use async_trait::async_trait;
use core_crypto::types::Hash;
use core_ethereum_db::traits::HoprCoreEthereumDbActions;
use core_ethereum_misc::errors::CoreEthereumError::{InvalidArguments, InvalidState};
use core_types::channels::{ChannelDirection, ChannelStatus};
use utils_log::{debug, error, info};
use utils_types::primitives::{Address, Balance, BalanceType};

use crate::errors::CoreEthereumActionsError::{
    BalanceTooLow, ClosureTimeHasNotElapsed, NotEnoughAllowance, PeerAccessDenied,
};
use crate::errors::{
    CoreEthereumActionsError::{ChannelAlreadyClosed, ChannelAlreadyExists, ChannelDoesNotExist},
    Result,
};
use crate::redeem::TicketRedeemActions;
use crate::transaction_queue::{Transaction, TransactionCompleted};
use crate::CoreEthereumActions;

#[cfg(all(feature = "wasm", not(test)))]
use utils_misc::time::wasm::current_timestamp;

#[cfg(any(not(feature = "wasm"), test))]
use utils_misc::time::native::current_timestamp;

/// Gathers all channel related on-chain actions.
#[async_trait(? Send)]
pub trait ChannelActions {
    /// Opens a channel to the given `destination` with the given `amount` staked.
    async fn open_channel(&self, destination: Address, amount: Balance) -> Result<TransactionCompleted>;

    /// Funds the given channel with the given `amount`
    async fn fund_channel(&self, channel_id: Hash, amount: Balance) -> Result<TransactionCompleted>;

    /// Closes the channel to counterparty in the given direction. Optionally can issue redeeming of all tickets in that channel.
    async fn close_channel(
        &self,
        counterparty: Address,
        direction: ChannelDirection,
        redeem_before_close: bool,
    ) -> Result<TransactionCompleted>;
}

#[async_trait(? Send)]
impl<Db: HoprCoreEthereumDbActions + Clone> ChannelActions for CoreEthereumActions<Db> {
    async fn open_channel(&self, destination: Address, amount: Balance) -> Result<TransactionCompleted> {
        if self.me == destination {
            return Err(InvalidArguments("cannot open channel to self".into()).into());
        }

        if amount.eq(&amount.of_same("0")) || amount.balance_type() != BalanceType::HOPR {
            return Err(InvalidArguments("invalid balance or balance type given".into()).into());
        }

        let allowance = self.db.read().await.get_staking_safe_allowance().await?;
        debug!("current staking safe allowance is {allowance}");
        if allowance.lt(&amount) {
            return Err(NotEnoughAllowance);
        }

        let hopr_balance = self.db.read().await.get_hopr_balance().await?;
        debug!("current node HOPR balance is {hopr_balance}");
        if hopr_balance.lt(&amount) {
            return Err(BalanceTooLow);
        }

        if self.db.read().await.is_network_registry_enabled().await?
            && !self.db.read().await.is_allowed_to_access_network(&destination).await?
        {
            return Err(PeerAccessDenied);
        }

        let maybe_channel = self.db.read().await.get_channel_x(&self.me, &destination).await?;
        if let Some(channel) = maybe_channel {
            debug!("already found existing {channel}");
            if channel.status != ChannelStatus::Closed {
                error!("channel to {destination} is already opened or pending to close");
                return Err(ChannelAlreadyExists);
            }
        }

        self.tx_sender.send(Transaction::OpenChannel(destination, amount)).await
    }

    async fn fund_channel(&self, channel_id: Hash, amount: Balance) -> Result<TransactionCompleted> {
        if amount.eq(&amount.of_same("0")) || amount.balance_type() != BalanceType::HOPR {
            return Err(InvalidArguments("invalid balance or balance type given".into()).into());
        }

        let allowance = self.db.read().await.get_staking_safe_allowance().await?;
        debug!("current staking safe allowance is {allowance}");
        if allowance.lt(&amount) {
            return Err(NotEnoughAllowance);
        }

        let hopr_balance = self.db.read().await.get_hopr_balance().await?;
        debug!("current node HOPR balance is {hopr_balance}");
        if hopr_balance.lt(&amount) {
            return Err(BalanceTooLow);
        }

        let maybe_channel = self.db.read().await.get_channel(&channel_id).await?;
        match maybe_channel {
            Some(channel) => {
                if channel.status == ChannelStatus::Open {
                    self.tx_sender.send(Transaction::FundChannel(channel, amount)).await
                } else {
                    Err(InvalidState(format!("channel {channel_id} is not opened")).into())
                }
            }
            None => Err(ChannelDoesNotExist),
        }
    }

    async fn close_channel(
        &self,
        counterparty: Address,
        direction: ChannelDirection,
        redeem_before_close: bool,
    ) -> Result<TransactionCompleted> {
        let maybe_channel = match direction {
            ChannelDirection::Incoming => self.db.read().await.get_channel_x(&counterparty, &self.me).await?,
            ChannelDirection::Outgoing => self.db.read().await.get_channel_x(&self.me, &counterparty).await?,
        };

        match maybe_channel {
            Some(channel) => {
                match channel.status {
                    ChannelStatus::Closed => Err(ChannelAlreadyClosed),
                    ChannelStatus::PendingToClose => {
                        info!(
                            "{channel} - remaining closure time is {:?}",
                            channel.remaining_closure_time(current_timestamp())
                        );
                        if channel.closure_time_passed(current_timestamp()) {
                            self.tx_sender.send(Transaction::CloseChannel(channel)).await
                        } else {
                            Err(ClosureTimeHasNotElapsed(
                                channel
                                    .remaining_closure_time(current_timestamp())
                                    .unwrap_or(u32::MAX as u64),
                            ))
                        }
                    }
                    ChannelStatus::Open => {
                        if redeem_before_close {
                            // TODO: trigger aggregation
                            // Do not await the redemption, just submit it to the queue
                            let redeemed = self.redeem_tickets_in_channel(&channel, false).await?.len();
                            info!("{redeemed} tickets will be redeemed before closing {channel}");
                        }

                        self.tx_sender.send(Transaction::CloseChannel(channel)).await
                        // TODO: emit "channel state change" event
                    }
                }
            }
            None => Err(ChannelDoesNotExist),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::channels::ChannelActions;
    use crate::errors::CoreEthereumActionsError;
    use crate::transaction_queue::{MockTransactionExecutor, TransactionQueue, TransactionResult};
    use crate::CoreEthereumActions;
    use async_lock::RwLock;
    use core_crypto::random::random_bytes;
    use core_crypto::types::Hash;
    use core_ethereum_db::db::CoreEthereumDb;
    use core_ethereum_db::traits::HoprCoreEthereumDbActions;
    use core_types::channels::{generate_channel_id, ChannelDirection, ChannelEntry, ChannelStatus};
    use mockall::Sequence;
    use std::ops::{Add, Sub};
    use std::sync::Arc;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use utils_db::db::DB;
    use utils_db::rusty::RustyLevelDbShim;
    use utils_types::primitives::{Address, Balance, BalanceType, Snapshot, U256};
    use utils_types::traits::BinarySerializable;

    #[async_std::test]
    async fn test_open_channel() {
        let _ = env_logger::builder().is_test(true).try_init();

        let self_addr = Address::random();
        let bob = Address::random();
        let stake = Balance::new(10_u32.into(), BalanceType::HOPR);
        let random_hash = Hash::new(&random_bytes::<{ Hash::SIZE }>());

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));
        db.write()
            .await
            .set_staking_safe_allowance(
                &Balance::new(10_000_000u64.into(), BalanceType::HOPR),
                &Snapshot::default(),
            )
            .await
            .unwrap();

        db.write()
            .await
            .set_hopr_balance(&Balance::new(5_000_000u64.into(), BalanceType::HOPR))
            .await
            .unwrap();

        db.write()
            .await
            .set_network_registry(false, &Snapshot::default())
            .await
            .unwrap();

        let mut tx_exec = MockTransactionExecutor::new();
        tx_exec
            .expect_open_channel()
            .times(1)
            .withf(move |dst, balance| bob.eq(dst) && stake.eq(balance))
            .returning(move |_, _| TransactionResult::OpenChannel {
                tx_hash: random_hash,
                channel_id: random_hash,
            });

        let tx_queue = TransactionQueue::new(db.clone(), Box::new(tx_exec));
        let tx_sender = tx_queue.new_sender();
        async_std::task::spawn_local(async move {
            tx_queue.transaction_loop().await;
        });

        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_sender.clone());

        let tx_res = actions.open_channel(bob, stake).await.unwrap().await.unwrap();

        match tx_res {
            TransactionResult::OpenChannel { tx_hash, channel_id } => {
                assert_eq!(random_hash, tx_hash, "tx hash must be equal");
                assert_eq!(random_hash, channel_id, "channel id must be equal");
            }
            _ => panic!("invalid or failed tx result"),
        }
    }

    #[async_std::test]
    async fn test_should_not_open_channel_again() {
        let _ = env_logger::builder().is_test(true).try_init();

        let self_addr = Address::random();
        let bob = Address::random();
        let stake = Balance::new(10_u32.into(), BalanceType::HOPR);

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));
        let tx_queue = TransactionQueue::new(db.clone(), Box::new(MockTransactionExecutor::new()));

        let channel = ChannelEntry::new(
            self_addr,
            bob,
            stake,
            U256::zero(),
            ChannelStatus::Open,
            U256::zero(),
            U256::zero(),
        );

        db.write()
            .await
            .set_staking_safe_allowance(
                &Balance::new(10_000_000u64.into(), BalanceType::HOPR),
                &Snapshot::default(),
            )
            .await
            .unwrap();

        db.write()
            .await
            .set_hopr_balance(&Balance::new(5_000_000u64.into(), BalanceType::HOPR))
            .await
            .unwrap();

        db.write()
            .await
            .update_channel_and_snapshot(&channel.get_id(), &channel, &Snapshot::default())
            .await
            .unwrap();

        db.write()
            .await
            .set_network_registry(false, &Snapshot::default())
            .await
            .unwrap();

        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_queue.new_sender());

        assert!(
            matches!(
                actions.open_channel(bob, stake).await.err().unwrap(),
                CoreEthereumActionsError::ChannelAlreadyExists
            ),
            "should fail when channel exists"
        );
    }

    #[async_std::test]
    async fn test_should_not_open_channel_to_self() {
        let _ = env_logger::builder().is_test(true).try_init();

        let self_addr = Address::random();
        let stake = Balance::new(10_u32.into(), BalanceType::HOPR);

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));
        let tx_queue = TransactionQueue::new(db.clone(), Box::new(MockTransactionExecutor::new()));

        db.write()
            .await
            .set_staking_safe_allowance(
                &Balance::new(10_000_000u64.into(), BalanceType::HOPR),
                &Snapshot::default(),
            )
            .await
            .unwrap();

        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_queue.new_sender());

        assert!(
            matches!(
                actions.open_channel(self_addr, stake).await.err().unwrap(),
                CoreEthereumActionsError::OtherError(_)
            ),
            "should not create channel to self"
        );
    }

    #[async_std::test]
    async fn test_open_should_not_allow_invalid_balance() {
        let _ = env_logger::builder().is_test(true).try_init();

        let self_addr = Address::random();
        let bob = Address::random();

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));
        let tx_queue = TransactionQueue::new(db.clone(), Box::new(MockTransactionExecutor::new()));

        db.write()
            .await
            .set_staking_safe_allowance(
                &Balance::new(10_000_000u64.into(), BalanceType::HOPR),
                &Snapshot::default(),
            )
            .await
            .unwrap();

        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_queue.new_sender());
        let stake = Balance::new(10_u32.into(), BalanceType::Native);
        assert!(
            matches!(
                actions.open_channel(bob, stake).await.err().unwrap(),
                CoreEthereumActionsError::OtherError(_)
            ),
            "should not allow invalid balance"
        );

        let stake = Balance::new(0_u32.into(), BalanceType::HOPR);

        assert!(
            matches!(
                actions.open_channel(bob, stake).await.err().unwrap(),
                CoreEthereumActionsError::OtherError(_)
            ),
            "should not allow invalid balance"
        );
    }

    #[async_std::test]
    async fn test_should_not_open_if_not_enough_allowance() {
        let _ = env_logger::builder().is_test(true).try_init();

        let self_addr = Address::random();
        let bob = Address::random();
        let stake = Balance::new(10_000_u32.into(), BalanceType::HOPR);

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));
        let tx_queue = TransactionQueue::new(db.clone(), Box::new(MockTransactionExecutor::new()));

        db.write()
            .await
            .set_staking_safe_allowance(&Balance::new(1000_u64.into(), BalanceType::HOPR), &Snapshot::default())
            .await
            .unwrap();

        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_queue.new_sender());

        assert!(
            matches!(
                actions.open_channel(bob, stake).await.err().unwrap(),
                CoreEthereumActionsError::NotEnoughAllowance
            ),
            "should fail when not enough allowance"
        );
    }

    #[async_std::test]
    async fn test_should_not_open_if_not_enough_token_balance() {
        let _ = env_logger::builder().is_test(true).try_init();

        let self_addr = Address::random();
        let bob = Address::random();
        let stake = Balance::new(10_000_u32.into(), BalanceType::HOPR);

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));
        let tx_queue = TransactionQueue::new(db.clone(), Box::new(MockTransactionExecutor::new()));

        db.write()
            .await
            .set_staking_safe_allowance(
                &Balance::new(1_000_000_u64.into(), BalanceType::HOPR),
                &Snapshot::default(),
            )
            .await
            .unwrap();

        db.write()
            .await
            .set_hopr_balance(&Balance::new(1_u64.into(), BalanceType::HOPR))
            .await
            .unwrap();

        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_queue.new_sender());

        assert!(
            matches!(
                actions.open_channel(bob, stake).await.err().unwrap(),
                CoreEthereumActionsError::BalanceTooLow
            ),
            "should fail when not enough token balance"
        );
    }

    #[async_std::test]
    async fn test_fund_channel() {
        let _ = env_logger::builder().is_test(true).try_init();

        let self_addr = Address::random();
        let bob = Address::random();
        let stake = Balance::new(10_u32.into(), BalanceType::HOPR);
        let random_hash = Hash::new(&random_bytes::<{ Hash::SIZE }>());

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));
        db.write()
            .await
            .set_staking_safe_allowance(
                &Balance::new(10_000_000u64.into(), BalanceType::HOPR),
                &Snapshot::default(),
            )
            .await
            .unwrap();

        db.write()
            .await
            .set_hopr_balance(&Balance::new(5_000_000u64.into(), BalanceType::HOPR))
            .await
            .unwrap();

        let channel = ChannelEntry::new(
            self_addr,
            bob,
            stake,
            U256::zero(),
            ChannelStatus::Open,
            U256::zero(),
            U256::zero(),
        );
        db.write()
            .await
            .update_channel_and_snapshot(&channel.get_id(), &channel, &Snapshot::default())
            .await
            .unwrap();

        let mut tx_exec = MockTransactionExecutor::new();
        tx_exec
            .expect_fund_channel()
            .times(1)
            .withf(move |dst, balance| channel.destination.eq(dst) && stake.eq(balance))
            .returning(move |_, _| TransactionResult::FundChannel { tx_hash: random_hash });

        let tx_queue = TransactionQueue::new(db.clone(), Box::new(tx_exec));
        let tx_sender = tx_queue.new_sender();
        async_std::task::spawn_local(async move {
            tx_queue.transaction_loop().await;
        });

        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_sender.clone());

        let tx_res = actions
            .fund_channel(channel.get_id(), stake)
            .await
            .unwrap()
            .await
            .unwrap();

        match tx_res {
            TransactionResult::FundChannel { tx_hash } => {
                assert_eq!(random_hash, tx_hash, "tx hash must be equal");
            }
            _ => panic!("invalid or failed tx result"),
        }
    }

    #[async_std::test]
    async fn test_should_not_fund_nonexistent_channel() {
        let _ = env_logger::builder().is_test(true).try_init();

        let self_addr = Address::random();
        let bob = Address::random();
        let channel_id = generate_channel_id(&self_addr, &bob);

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));
        let tx_queue = TransactionQueue::new(db.clone(), Box::new(MockTransactionExecutor::new()));

        db.write()
            .await
            .set_staking_safe_allowance(
                &Balance::new(10_000_000u64.into(), BalanceType::HOPR),
                &Snapshot::default(),
            )
            .await
            .unwrap();

        db.write()
            .await
            .set_hopr_balance(&Balance::new(5_000_000u64.into(), BalanceType::HOPR))
            .await
            .unwrap();

        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_queue.new_sender());
        let stake = Balance::new(10_u32.into(), BalanceType::HOPR);
        assert!(
            matches!(
                actions.fund_channel(channel_id, stake).await.err().unwrap(),
                CoreEthereumActionsError::ChannelDoesNotExist
            ),
            "should fail when channel does not exist"
        );
    }

    #[async_std::test]
    async fn test_fund_should_not_allow_invalid_balance() {
        let _ = env_logger::builder().is_test(true).try_init();

        let self_addr = Address::random();
        let bob = Address::random();
        let channel_id = generate_channel_id(&self_addr, &bob);

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));
        let tx_queue = TransactionQueue::new(db.clone(), Box::new(MockTransactionExecutor::new()));

        db.write()
            .await
            .set_staking_safe_allowance(
                &Balance::new(10_000_000u64.into(), BalanceType::HOPR),
                &Snapshot::default(),
            )
            .await
            .unwrap();

        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_queue.new_sender());
        let stake = Balance::new(10_u32.into(), BalanceType::Native);
        assert!(
            matches!(
                actions.open_channel(bob, stake).await.err().unwrap(),
                CoreEthereumActionsError::OtherError(_)
            ),
            "should not allow invalid balance"
        );

        let stake = Balance::new(0_u32.into(), BalanceType::HOPR);
        assert!(
            matches!(
                actions.fund_channel(channel_id, stake).await.err().unwrap(),
                CoreEthereumActionsError::OtherError(_)
            ),
            "should not allow invalid balance"
        );
    }

    #[async_std::test]
    async fn test_should_not_fund_if_not_enough_allowance() {
        let _ = env_logger::builder().is_test(true).try_init();

        let self_addr = Address::random();
        let bob = Address::random();
        let channel_id = generate_channel_id(&self_addr, &bob);

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));
        let tx_queue = TransactionQueue::new(db.clone(), Box::new(MockTransactionExecutor::new()));

        db.write()
            .await
            .set_staking_safe_allowance(&Balance::new(1000_u64.into(), BalanceType::HOPR), &Snapshot::default())
            .await
            .unwrap();

        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_queue.new_sender());
        let stake = Balance::new(10_000_u32.into(), BalanceType::HOPR);
        assert!(
            matches!(
                actions.fund_channel(channel_id, stake).await.err().unwrap(),
                CoreEthereumActionsError::NotEnoughAllowance
            ),
            "should fail when not enough allowance"
        );
    }

    #[async_std::test]
    async fn test_should_not_fund_if_not_enough_balance() {
        let _ = env_logger::builder().is_test(true).try_init();

        let self_addr = Address::random();
        let bob = Address::random();
        let channel_id = generate_channel_id(&self_addr, &bob);

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));
        let tx_queue = TransactionQueue::new(db.clone(), Box::new(MockTransactionExecutor::new()));

        db.write()
            .await
            .set_staking_safe_allowance(
                &Balance::new(1_000_000_u64.into(), BalanceType::HOPR),
                &Snapshot::default(),
            )
            .await
            .unwrap();

        db.write()
            .await
            .set_hopr_balance(&Balance::new(1u64.into(), BalanceType::HOPR))
            .await
            .unwrap();

        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_queue.new_sender());
        let stake = Balance::new(10_000_u32.into(), BalanceType::HOPR);
        assert!(
            matches!(
                actions.fund_channel(channel_id, stake).await.err().unwrap(),
                CoreEthereumActionsError::BalanceTooLow
            ),
            "should fail when not enough balance"
        );
    }

    async fn base_test_channel_close(direction: ChannelDirection) {
        let self_addr = Address::random();
        let bob = Address::random();
        let stake = Balance::new(10_u32.into(), BalanceType::HOPR);
        let random_hash = Hash::new(&random_bytes::<{ Hash::SIZE }>());

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));

        let mut channel = match direction {
            ChannelDirection::Incoming => ChannelEntry::new(
                bob,
                self_addr,
                stake,
                U256::zero(),
                ChannelStatus::Open,
                U256::zero(),
                U256::zero(),
            ),
            ChannelDirection::Outgoing => ChannelEntry::new(
                self_addr,
                bob,
                stake,
                U256::zero(),
                ChannelStatus::Open,
                U256::zero(),
                U256::zero(),
            ),
        };

        db.write()
            .await
            .update_channel_and_snapshot(&channel.get_id(), &channel, &Snapshot::default())
            .await
            .unwrap();

        let mut tx_exec = MockTransactionExecutor::new();
        let mut seq = Sequence::new();
        tx_exec
            .expect_close_channel_initialize()
            .times(1)
            .in_sequence(&mut seq)
            .withf(move |src, dst| match direction {
                ChannelDirection::Incoming => self_addr.eq(dst) && bob.eq(src),
                ChannelDirection::Outgoing => self_addr.eq(src) && bob.eq(dst),
            })
            .returning(move |_, _| TransactionResult::CloseChannel {
                tx_hash: random_hash,
                status: ChannelStatus::PendingToClose,
            });

        tx_exec
            .expect_close_channel_finalize()
            .times(1)
            .in_sequence(&mut seq)
            .withf(move |src, dst| match direction {
                ChannelDirection::Incoming => self_addr.eq(dst) && bob.eq(src),
                ChannelDirection::Outgoing => self_addr.eq(src) && bob.eq(dst),
            })
            .returning(move |_, _| TransactionResult::CloseChannel {
                tx_hash: random_hash,
                status: ChannelStatus::Closed,
            });

        let tx_queue = TransactionQueue::new(db.clone(), Box::new(tx_exec));
        let tx_sender = tx_queue.new_sender();
        async_std::task::spawn_local(async move {
            tx_queue.transaction_loop().await;
        });

        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_sender.clone());

        let tx_res = actions
            .close_channel(bob, direction, false)
            .await
            .unwrap()
            .await
            .unwrap();

        match tx_res {
            TransactionResult::CloseChannel { tx_hash, status } => {
                assert_eq!(random_hash, tx_hash, "tx hash must be equal");
                assert_eq!(ChannelStatus::PendingToClose, status, "status must be equal");
            }
            _ => panic!("invalid or failed tx result"),
        }

        // Transition the channel to the PendingToClose state with the closure time already elapsed
        channel.status = ChannelStatus::PendingToClose;
        channel.closure_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .sub(Duration::from_secs(10))
            .as_secs()
            .into();

        db.write()
            .await
            .update_channel_and_snapshot(&channel.get_id(), &channel, &Snapshot::default())
            .await
            .unwrap();

        let tx_res = actions
            .close_channel(bob, direction, false)
            .await
            .unwrap()
            .await
            .unwrap();

        match tx_res {
            TransactionResult::CloseChannel { tx_hash, status } => {
                assert_eq!(random_hash, tx_hash, "tx hash must be equal");
                assert_eq!(ChannelStatus::Closed, status, "status must be equal");
            }
            _ => panic!("invalid or failed tx result"),
        }
    }

    #[async_std::test]
    async fn test_close_channel_outgoing() {
        let _ = env_logger::builder().is_test(true).try_init();
        base_test_channel_close(ChannelDirection::Outgoing).await
    }

    #[async_std::test]
    async fn test_close_channel_incoming() {
        let _ = env_logger::builder().is_test(true).try_init();
        base_test_channel_close(ChannelDirection::Incoming).await
    }

    #[async_std::test]
    async fn test_should_not_close_when_closure_time_did_not_elapse() {
        let self_addr = Address::random();
        let bob = Address::random();
        let stake = Balance::new(10_u32.into(), BalanceType::HOPR);

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));

        let channel = ChannelEntry::new(
            self_addr,
            bob,
            stake,
            U256::zero(),
            ChannelStatus::PendingToClose,
            U256::zero(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .add(Duration::from_secs(100))
                .as_secs()
                .into(),
        );

        db.write()
            .await
            .update_channel_and_snapshot(&channel.get_id(), &channel, &Snapshot::default())
            .await
            .unwrap();

        let tx_queue = TransactionQueue::new(db.clone(), Box::new(MockTransactionExecutor::new()));

        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_queue.new_sender());

        assert!(
            matches!(
                actions
                    .close_channel(bob, ChannelDirection::Outgoing, false)
                    .await
                    .err()
                    .unwrap(),
                CoreEthereumActionsError::ClosureTimeHasNotElapsed(_)
            ),
            "should fail when the channel closure period did not elapse"
        );
    }

    #[async_std::test]
    async fn test_should_not_close_nonexistent_channel() {
        let _ = env_logger::builder().is_test(true).try_init();

        let self_addr = Address::random();
        let bob = Address::random();

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));
        let tx_queue = TransactionQueue::new(db.clone(), Box::new(MockTransactionExecutor::new()));
        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_queue.new_sender());

        assert!(
            matches!(
                actions
                    .close_channel(bob, ChannelDirection::Outgoing, false)
                    .await
                    .err()
                    .unwrap(),
                CoreEthereumActionsError::ChannelDoesNotExist
            ),
            "should fail when channel does not exist"
        );
    }

    #[async_std::test]
    async fn test_should_not_close_closed_channel() {
        let _ = env_logger::builder().is_test(true).try_init();

        let self_addr = Address::random();
        let bob = Address::random();
        let stake = Balance::new(10_u32.into(), BalanceType::HOPR);

        let db = Arc::new(RwLock::new(CoreEthereumDb::new(
            DB::new(RustyLevelDbShim::new_in_memory()),
            self_addr,
        )));
        let tx_queue = TransactionQueue::new(db.clone(), Box::new(MockTransactionExecutor::new()));
        let actions = CoreEthereumActions::new(self_addr, db.clone(), tx_queue.new_sender());

        let channel = ChannelEntry::new(
            self_addr,
            bob,
            stake,
            U256::zero(),
            ChannelStatus::Closed,
            U256::zero(),
            U256::zero(),
        );
        db.write()
            .await
            .update_channel_and_snapshot(&channel.get_id(), &channel, &Snapshot::default())
            .await
            .unwrap();

        assert!(
            matches!(
                actions
                    .close_channel(bob, ChannelDirection::Outgoing, false)
                    .await
                    .err()
                    .unwrap(),
                CoreEthereumActionsError::ChannelAlreadyClosed
            ),
            "should fail when channel is already closed"
        );
    }
}

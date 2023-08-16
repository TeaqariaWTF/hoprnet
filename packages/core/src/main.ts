import path from 'path'

import { debug, LevelDb, ChainKeypair, OffchainKeypair } from '@hoprnet/hopr-utils'

import HoprCoreEthereum from '@hoprnet/hopr-core-ethereum'

import { Hopr, type HoprOptions } from './index.js'
import { getContractData } from './network.js'
import { Database, core_hopr_initialize_crate, Address as Packet_Address } from '../lib/core_hopr.js'
core_hopr_initialize_crate()

const log = debug(`hopr-core:create-hopr`)

/*
 * General function to create a HOPR node given an identity an
 * range of options.
 * @param peerId:PeerId - Identity used by the HOPR node
 * @param options:HoprOptions - Required options to create node
 * @returns {Hopr} - HOPR node
 */
export async function createHoprNode(
  chainKeypair: ChainKeypair,
  packetKeypair: OffchainKeypair,
  options: HoprOptions,
  automaticChainCreation = true
): Promise<Hopr> {
  let levelDb = new LevelDb()

  try {
    const dbPath = path.join(options.dataPath, 'db')
    await levelDb.init(options.createDbIfNotExist, dbPath, options.forceCreateDB, options.network.id)

    // Dump entire database to a file if given by the env variable
    const dump_file = process.env.DB_DUMP ?? ''
    if (dump_file.length > 0) {
      await levelDb.dump(dump_file)
    }
  } catch (err: unknown) {
    log(`failed init db:`, err)
    throw err
  }

  let db = new Database(levelDb, chainKeypair.public().to_address())

  // if safe address or module address is not provided, replace with values stored in the db
  log(`options.safeModule.safeAddress: ${options.safeModule.safeAddress}`)
  log(`options.safeModule.moduleAddress: ${options.safeModule.moduleAddress}`)
  const safeAddress =
    options.safeModule.safeAddress ?? Packet_Address.deserialize((await db.get_staking_safe_address()).serialize())
  const moduleAddress =
    options.safeModule.moduleAddress ?? Packet_Address.deserialize((await db.get_staking_module_address()).serialize())
  if (!safeAddress || !moduleAddress) {
    log(`failed to provide safe or module address:`)
    throw new Error('Hopr Node must be initialized with safe and module address')
  }

  log(`using provider URL: ${options.network.chain.default_provider}`)

  // get contract data for the given environment id and pass it on to create chain wrapper
  const resolvedContractAddresses = getContractData(options.network.id)
  log(`[DEBUG] resolvedContractAddresses ${options.network.id} ${JSON.stringify(resolvedContractAddresses, null, 2)}`)

  await HoprCoreEthereum.createInstance(
    db,
    chainKeypair,
    {
      chainId: options.network.chain.chain_id,
      network: options.network.id,
      maxFeePerGas: options.network.chain.max_fee_per_gas,
      maxPriorityFeePerGas: options.network.chain.max_priority_fee_per_gas,
      chain: options.network.chain.id,
      provider: options.network.chain.default_provider
    },
    {
      safeTransactionServiceProvider: options.safeModule.safeTransactionServiceProvider,
      safeAddress,
      moduleAddress
    },
    resolvedContractAddresses,
    automaticChainCreation
  )

  // // Initialize connection to the blockchain
  // await chain.initializeChainWrapper(resolvedContractAddresses)

  return new Hopr(chainKeypair, packetKeypair, db, options)
}

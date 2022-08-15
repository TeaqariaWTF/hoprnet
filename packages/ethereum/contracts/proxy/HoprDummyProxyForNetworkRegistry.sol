// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

import '@openzeppelin/contracts/access/Ownable.sol';
import '../IHoprNetworkRegistryRequirement.sol';

/**
 * @dev Dummy roxy which return true if an address is registered by the owner, when isAllAllowed is false.
 * It allows all the accounts when isAllAllowed is set to true. By default isAllAllowed is false.
 */
contract HoprDummyProxyForNetworkRegistry is IHoprNetworkRegistryRequirement, Ownable {
  mapping(address => bool) registeredAccounts;
  event AccountRegistered(address indexed account);
  event AccountDeregistered(address indexed account);
  bool public isAllAllowed;

  event AllowAllAccountsEligible(bool isAllowed);

  constructor(address newOwner) {
    _transferOwnership(newOwner);
    isAllAllowed = false;
    emit AllowAllAccountsEligible(false);
  }

  /**
   * @dev Checks if the provided account is registered by the owner
   * @param account address of the account that runs a hopr node
   */
  function isRequirementFulfilled(address account) external view returns (bool) {
    if (isAllAllowed) {
      return true;
    } else {
      return registeredAccounts[account];
    }
  }

  /**
   * @dev Update the global toggle that allows all the accounts to be eligible
   */
  function updateAllowAll(bool _updatedAllow) external onlyOwner {
    if (isAllAllowed == _updatedAllow) {
      return;
    }
    isAllAllowed = _updatedAllow;
    emit AllowAllAccountsEligible(_updatedAllow);
  }

  /**
   * @dev Owner add accounts onto the registry list in batch.
   * @param accounts addresses to be removed from the registry
   */
  function ownerBatchAddAccounts(address[] calldata accounts) external onlyOwner {
    for (uint256 index = 0; index < accounts.length; index++) {
      _addAccount(accounts[index]);
    }
  }

  /**
   * @dev Owner removes from list of eligible NFTs in batch.
   * @param accounts addresses to be removed from the registry
   */
  function ownerBatchRemoveAccounts(address[] calldata accounts) external onlyOwner {
    for (uint256 index = 0; index < accounts.length; index++) {
      _removeAccount(accounts[index]);
    }
  }

  /**
   * @dev Owner add account onto the registry list
   * @param account address to be added onto the registry
   */
  function ownerAddAccount(address account) external onlyOwner {
    _addAccount(account);
  }

  /**
   * @dev Owner move account from the registry list
   * @param account address to be removed from the registry
   */
  function ownerRemoveAccount(address account) external onlyOwner {
    _removeAccount(account);
  }

  /**
   * @dev add account onto the registry list
   * @param account address to be added into the registry
   */
  function _addAccount(address account) private {
    if (!registeredAccounts[account]) {
      registeredAccounts[account] = true;
      emit AccountRegistered(account);
    }
  }

  /**
   * @dev remove account from the registry list
   * @param account address to be removed from the registry
   */
  function _removeAccount(address account) private {
    if (registeredAccounts[account]) {
      delete registeredAccounts[account];
      emit AccountDeregistered(account);
    }
  }
}

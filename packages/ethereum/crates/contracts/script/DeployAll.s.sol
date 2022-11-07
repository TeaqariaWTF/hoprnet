// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.6.0 <0.9.0;
pragma abicoder v2;

import "forge-std/Script.sol";
import "../test/utils/Deploy.sol";
import "./utils/EnvironmentConfig.s.sol";

contract DeployAllContractsScript is Script, EnvironmentConfig, ERC1820RegistryFixture {
    function run() external {
        // 1. Environment check
        // get envirionment of the script
        getEnvrionment();
        // read records of deployed files
        readCurrentEnvironment();
        // Halt if ERC1820Registry has not been deployed.
        mustHaveErc1820Registry();

        // 2. Get deployer private key
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address deployerAddress = vm.addr(deployerPrivateKey);
        vm.startBroadcast(deployerPrivateKey);

        // 3. Deploy
        // 3.1. HoprToken Contract
        // Only deploy Token contract when no deployed one is detected. 
        // E.g. always in development envirionment, or should a new token contract be introduced in staging/production.
        if (currentEnvironmentType == EnvironmentType.DEVELOPMENT || !isValidAddress(currentEnvironmentDetail.hoprTokenContractAddress)) {
            // deploy token contract
            currentEnvironmentDetail.hoprTokenContractAddress = deployCode("HoprToken.sol");
            // grant deployer minter role
            (bool successGrantMinterRole, ) = currentEnvironmentDetail.hoprTokenContractAddress.call(abi.encodeWithSignature("grantRole(bytes32,address)", keccak256("MINTER_ROLE"), deployerAddress));
            if (!successGrantMinterRole) {
                emit log_string("Cannot grantMinterRole");
            }
            // mint some tokens to the deployer
            (bool successMintTokens, ) = currentEnvironmentDetail.hoprTokenContractAddress.call(abi.encodeWithSignature("mint(address,uint256,bytes,bytes)", deployerAddress, 130000000 ether, hex"00", hex"00"));
            if (!successMintTokens) {
                emit log_string("Cannot mint tokens");
            }
        }

        // 3.2. HoprChannels Contract
        // Only deploy Channels contract when no deployed one is detected. 
        // E.g. always in development envirionment, or should a new channel contract be introduced in staging/production per meta environment. 
        if (currentEnvironmentType == EnvironmentType.DEVELOPMENT || !isValidAddress(currentEnvironmentDetail.hoprChannelsContractAddress)) {
            // deploy channels contract
            uint256 closure = currentEnvironmentType == EnvironmentType.DEVELOPMENT ? 15 : 5 * 60;
            currentEnvironmentDetail.hoprChannelsContractAddress = deployCode("HoprChannels.sol", abi.encode(currentEnvironmentDetail.hoprTokenContractAddress, closure));
        }

        // 3.3. xHoprToken Contract
        // Only deploy Token contract when no deployed one is detected. 
        // E.g. always in development envirionment, or should a new token contract be introduced in staging. 
        // Production contract should remain 0xD057604A14982FE8D88c5fC25Aac3267eA142a08 TODO: Consider force check on this address
        if (currentEnvironmentType == EnvironmentType.DEVELOPMENT || !isValidAddress(currentEnvironmentDetail.xhoprTokenContractAddress)) {
            // deploy xtoken contract
            currentEnvironmentDetail.xhoprTokenContractAddress = deployCode("ERC677Mock.sol");
        }
        
        // 3.4. HoprBoost Contract
        // Only deploy Boost contract when no deployed one is detected. 
        // E.g. always in development envirionment, or should a new token contract be introduced in staging. 
        // Production contract should remain 0x43d13D7B83607F14335cF2cB75E87dA369D056c7 TODO: Consider force check on this address
        if (currentEnvironmentType == EnvironmentType.DEVELOPMENT || !isValidAddress(currentEnvironmentDetail.hoprBoostContractAddress)) {
            // deploy boost contract
            currentEnvironmentDetail.hoprBoostContractAddress = deployCode("HoprBoost.sol", abi.encode(deployerAddress, ""));
        }

        // 3.5. HoprStake Contract
        // Only deply HoprStake contract (of the latest season) when no deployed one is detected.
        // E.g. always in development environment, or should a new stake contract be introduced in staging.
        if (currentEnvironmentType == EnvironmentType.DEVELOPMENT || !isValidAddress(currentEnvironmentDetail.stakeContractAddress)) {
            // build the staking season artifact name, based on the stake season number specified in the contract-addresses.json
            string memory stakeArtifactName = string(abi.encodePacked("HoprStakeSeason", vm.toString(currentEnvironmentDetail.stakeSeason), ".sol"));
            // deploy stake contract
            currentEnvironmentDetail.stakeContractAddress = deployCode(stakeArtifactName, abi.encode(deployerAddress, currentEnvironmentDetail.hoprBoostContractAddress, currentEnvironmentDetail.xhoprTokenContractAddress, currentEnvironmentDetail.hoprTokenContractAddress));
        }

        // 3.6. NetworkRegistryProxy Contract
        // Only deploy NetworkRegistryProxy contract when no deployed one is detected.
        // E.g. Always in development environment, or should a new NetworkRegistryProxy contract be introduced in staging/production
        if (currentEnvironmentType == EnvironmentType.DEVELOPMENT) {
            // deploy DummyProxy in DEVELOPMENT envirionment
            currentEnvironmentDetail.networkRegistryProxyContractAddress = deployCode("HoprDummyProxyForNetworkRegistry.sol", abi.encode(deployerAddress));
        } else if (!isValidAddress(currentEnvironmentDetail.networkRegistryProxyContractAddress)) {
            // deploy StakingProxy in other envrionment types, if no proxy contract is given.
            currentEnvironmentDetail.networkRegistryProxyContractAddress = deployCode("HoprStakingProxyForNetworkRegistry.sol", abi.encode(currentEnvironmentDetail.stakeContractAddress, deployerAddress, 1000 ether));
            // TODO: If needed, add `eligibleNftTypeAndRank`. Only execute this transaction when NR accepts accounts with staking amount above certain threshold
            // Add `Network_registry` NFT (index. 26) (`developer` and `community`) into `specialNftTypeAndRank` TODO: extend this array if more NR NFTs are to be included
            bytes memory builtPayload = buildBatchRegisterSpecialNrNft(); // This payload is built because default abi.encode returns different value (no size info) when array is static.
            (bool successOwnerBatchAddSpecialNftTypeAndRank, ) = currentEnvironmentDetail.networkRegistryProxyContractAddress.call(builtPayload);
            if (!successOwnerBatchAddSpecialNftTypeAndRank) {
                emit log_string("Cannot ownerBatchAddSpecialNftTypeAndRank");
                emit log_bytes(builtPayload);
            }
        } else {
            // When a NetworkRegistryProxy contract is provided, check if its `stakeContract` matches with the latest stakeContractAddress. 
            (bool successReadStakeContract, bytes memory returndataStakeContract) = currentEnvironmentDetail.networkRegistryProxyContractAddress.staticcall(abi.encodeWithSignature("stakeContract()"));
            if (!successReadStakeContract) {
                emit log_string("Cannot read stakeContract");
            }
            address linkedStakeContract = abi.decode(returndataStakeContract, (address));
            // Check if the current sender is NetworkRegistryProxy owner. 
            (bool successReadProxyOwner, bytes memory returndataProxyOwner) = currentEnvironmentDetail.networkRegistryProxyContractAddress.staticcall(abi.encodeWithSignature("owner()"));
            if (!successReadProxyOwner) {
                emit log_string("Cannot read owner");
            }
            address proxyOwner = abi.decode(returndataProxyOwner, (address));
            // When a mismatch is deteced and the deployer (transaction sender) is the owner, update the `stakeContract` with the latest staking contract address
            if (linkedStakeContract != currentEnvironmentDetail.stakeContractAddress && proxyOwner == deployerAddress) {
                (bool successUpdateStakeContract, ) = currentEnvironmentDetail.networkRegistryProxyContractAddress.call(abi.encodeWithSignature("updateStakeContract(address)", currentEnvironmentDetail.stakeContractAddress));
                if (!successUpdateStakeContract) {
                    emit log_string("Cannot updateStakeContract");
                }
            }
        }

        // 3.7. NetworkRegistry Contract
        // Only deploy NetworkRegistrycontract when no deployed one is detected.
        // E.g. Always in development environment, or should a new NetworkRegistryProxy contract be introduced in staging/production
        if (currentEnvironmentType == EnvironmentType.DEVELOPMENT || !isValidAddress(currentEnvironmentDetail.networkRegistryContractAddress)) {
            // deploy NetworkRegistry contract
            currentEnvironmentDetail.networkRegistryContractAddress = deployCode("HoprNetworkRegistry.sol", abi.encode(currentEnvironmentDetail.networkRegistryProxyContractAddress, deployerAddress));
            // NetworkRegistry should be enabled (default behavior) in staging/production, and disabled in development
            if (currentEnvironmentType == EnvironmentType.DEVELOPMENT) {
                (bool successDisableRegistry, ) = currentEnvironmentDetail.networkRegistryContractAddress.call(abi.encodeWithSignature("disableRegistry()"));
                if (!successDisableRegistry) {
                    emit log_string("Cannot disableRegistry");
                }
            }
        } else {
            // When a NetworkRegistry contract is provided, check if its `requirementImplementation` matches with the latest NetworkRegistryProxy. 
            (bool successReadRequirementImplementation, bytes memory returndataRequirementImplementation) = currentEnvironmentDetail.networkRegistryContractAddress.staticcall(abi.encodeWithSignature("requirementImplementation()"));
            if (!successReadRequirementImplementation) {
                emit log_string("Cannot read RequirementImplementation");
            }
            address requirementImplementation = abi.decode(returndataRequirementImplementation, (address));
            // Check if the current sender is NetworkRegistry owner. 
            (bool successReadOwner, bytes memory returndataOwner) = currentEnvironmentDetail.networkRegistryContractAddress.staticcall(abi.encodeWithSignature("owner()"));
            if (!successReadOwner) {
                emit log_string("Cannot read NetworkRegistry contract owner");
            }
            address networkRegistryOwner = abi.decode(returndataOwner, (address));
            // When a mismatch is deteced and the deployer (transaction sender) is the owner, update the `requirementImplementation` with the latest NetworkRegistryProxy address
            if (requirementImplementation != currentEnvironmentDetail.networkRegistryProxyContractAddress && networkRegistryOwner == deployerAddress) {
                (bool successUpdateImplementation, ) = currentEnvironmentDetail.networkRegistryContractAddress.call(abi.encodeWithSignature("updateRequirementImplementation(address)", currentEnvironmentDetail.networkRegistryProxyContractAddress));
                if (!successUpdateImplementation) {
                    emit log_string("Cannot updateRequirementImplementation");
                }
            }
        }

        // write to file
        vm.stopBroadcast();

        // FIXME: to write to a json file
        displayCurrentEnvironmentDetail();
    }

    /**
     * @dev Helper function to build payload for "ownerBatchAddSpecialNftTypeAndRank(uint256[],string[],uint256[])"
     * By default, it adds `Network_registry` NFT (index. 26) (`developer` and `community`) 
     * It's possible to extend this array if more NR NFTs are issued
     */
    function buildBatchRegisterSpecialNrNft() private returns(bytes memory) {
        // "Network_registry" type
        uint256[] memory typeIndex = new uint256[](2);
        typeIndex[0] = 26;
        typeIndex[1] = 26;
        // "developer" and "community" rank
        string[] memory ranks = new string[](2);
        ranks[0] = "developer";
        ranks[1] = "community";
        // max. number of allowed registration
        uint256[] memory maxAllowedReg = new uint256[](2);
        maxAllowedReg[0] = type(uint256).max;
        maxAllowedReg[1] = 1;

        return abi.encodeWithSignature("ownerBatchAddSpecialNftTypeAndRank(uint256[],string[],uint256[])", typeIndex, ranks, maxAllowedReg);
    }
}

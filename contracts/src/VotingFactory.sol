// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./Voting.sol";
import "./Errors.sol";

contract VotingFactory {
    enum CeremonyType {
        Binary,
        Auction,
        MultipleChoice
    }

    event Deployed(address indexed contractAddress, CeremonyType votingType);

    mapping(bytes32 => address) public contracts;

    function deployVotingContract(
        bytes32 salt,
        CeremonyType ceremonyType,
        address verifier,
        uint256 submissionDeadline,
        uint256 tallyDeadline,
        uint256 merkleRoot,
        uint256 ceremonyId,
        uint256 stateRoot
    ) external returns (address) {
        if (contracts[salt] != address(0)) revert SaltAlreadyUsed(salt);

        bytes memory bytecode = getCeremonyBytecode(ceremonyType);

        address deployedAddress;
        assembly {
            deployedAddress := create2(0, add(bytecode, 0x20), mload(bytecode), salt)
        }

        if (deployedAddress == address(0)) revert DeploymentFailed();

        Voting(deployedAddress).initialize(
            verifier, submissionDeadline, tallyDeadline, merkleRoot, ceremonyId, stateRoot
        );

        contracts[salt] = deployedAddress;

        emit Deployed(deployedAddress, ceremonyType);

        return deployedAddress;
    }

    // function deployAuction(
    // ) external returns (address) {
    //     // TODO
    // }

    // function deployMultipleChoice(
    // ) external returns (address) {
    //     // TODO
    // }

    function getCeremonyBytecode(CeremonyType votingType) internal pure returns (bytes memory) {
        if (votingType == CeremonyType.Binary) {
            return type(Voting).creationCode;
        } else {
            revert InvalidCeremonyType();
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./Voting.sol";
import "./Auction.sol";
import "./Errors.sol";

contract Factory {
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
        // uint256 castingDeadline,
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

    function deployAuctionContract(
        bytes32 _salt,
        CeremonyType _ceremonyType,
        address _verifier,
        uint256 _biddingDealine,
        uint256 _submissionDeadline,
        uint256 _tallyDeadline,
        uint256 _merkleRoot,
        uint256 _ceremonyId,
        uint256 _stateRoot,
        address _ceremonyToken,
        uint256 _maxWinners
    ) external returns (address) {
        if (contracts[_salt] != address(0)) revert SaltAlreadyUsed(_salt);
        bytes memory bytecode = getCeremonyBytecode(_ceremonyType);

        address deployedAddress;

        assembly {
            deployedAddress := create2(0, add(bytecode, 0x20), mload(bytecode), _salt)
        }

        Auction(deployedAddress).initialize(
            _verifier,
            _biddingDealine,
            _submissionDeadline,
            _tallyDeadline,
            _merkleRoot,
            _ceremonyId,
            _stateRoot,
            _ceremonyToken,
            _maxWinners
        );

        contracts[_salt] = deployedAddress;
        emit Deployed(deployedAddress, _ceremonyType);

        return deployedAddress;
    }

    // function deployMultipleChoice(
    // ) external returns (address) {
    //     // TODO
    // }

    function getCeremonyBytecode(CeremonyType votingType) internal pure returns (bytes memory) {
        if (votingType == CeremonyType.Binary) {
            return type(Voting).creationCode;
        } else if (votingType == CeremonyType.Auction) {
            return type(Auction).creationCode;
        } else {
            revert InvalidCeremonyType();
        }
    }
}

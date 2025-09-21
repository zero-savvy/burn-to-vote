// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Factory} from "../src/Factory.sol";
import {Groth16Verifier} from "../src/verifier.sol";

contract VotingScript is Script {
    Factory public factory;
    Groth16Verifier public verifier;

    function setUp() public {}

    function run(
        bytes32 salt,
        uint256 submissionDeadline,
        uint256 tallyDeadline,
        uint256 ceremonyId,
        uint256 merkleRoot,
        uint256 stateRoot
    ) public {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(privateKey);

        uint256 b = vm.addr(privateKey).balance;
        console.log("balance", b);

        verifier = new Groth16Verifier();
        votingFactory = new Factory();
        address voting = votingFactory.deployVotingContract(
            salt,
            Factory.CeremonyType.Binary,
            address(verifier),
            submissionDeadline,
            tallyDeadline,
            merkleRoot,
            ceremonyId,
            stateRoot
        );

        vm.stopBroadcast();
        console.logAddress(address(voting));
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Voting} from "../src/Voting.sol";
import {Groth16Verifier} from "../src/verifier.sol";

contract VotingScript is Script {
    Voting public voting;
    Groth16Verifier public verifier;

    function setUp() public {}

    function run() public {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(privateKey);

        // uint256 base_time = 1000;
        // vm.warp(base_time);

        uint256 votingTime = block.timestamp + 10000;
        uint256 tallyTime = block.timestamp + 20000;
        uint256 ceremeny_id = 16325618567567054338;
        uint256 mt = 6279442489816579343175600576641714715845361010123760250696645575814262324581;
        uint state_root = 9133689217370487228376476215699836963181592635914481284078419964281904630813;

        verifier = new Groth16Verifier();
        voting = new Voting(address(verifier), votingTime, tallyTime, mt, ceremeny_id, state_root);

        vm.stopBroadcast();
        console.logAddress(address(voting));
    }
}
// forge script VotingScript --rpc-url http://localhost:8545 --broadcast

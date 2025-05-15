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


        uint256 voting_time = block.timestamp + 10000;
        uint256 tally_time = block.timestamp + 20000;

        verifier = new Groth16Verifier();
        voting = new Voting(address(verifier),voting_time,tally_time);

        vm.stopBroadcast();
        console.logAddress(address(voting));


    }
}
// forge script VotingScript --rpc-url http://localhost:8545 --broadcast

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
        uint256 ceremeny_id = 14564730128827983570;
        uint256 mt = 16140668641613092893634150840665751551734694739321141751642452107309003732465;

        verifier = new Groth16Verifier();
        voting = new Voting(address(verifier), voting_time, tally_time, mt, ceremeny_id);

        vm.stopBroadcast();
        console.logAddress(address(voting));
    }
}
// forge script VotingScript --rpc-url http://localhost:8545 --broadcast

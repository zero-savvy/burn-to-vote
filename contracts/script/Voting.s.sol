// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Voting} from "../src/Voting.sol";

contract VotingScript is Script {
    Voting public voting;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        voting = new Voting();

        vm.stopBroadcast();
    }
}

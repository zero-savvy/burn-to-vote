// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Voting} from "../src/Voting.sol";
import {Groth16Verifier} from "../src/verifier.sol";

contract VotingScript is Script {
    Voting public voting;
    Groth16Verifier public verifier;

    function setUp() public {}

    function run(
        uint256 votingTime,
        uint256 tallyTime,
        uint256 ceremony_id,
        uint256 mt,
        uint256 state_root
    ) public {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(privateKey);

        uint256 b = vm.addr(privateKey).balance;
        console.log("balance", b);

        verifier = new Groth16Verifier();
        voting = new Voting(address(verifier), votingTime, tallyTime, mt, ceremony_id, state_root);

        vm.stopBroadcast();
        console.logAddress(address(voting));
    }
}

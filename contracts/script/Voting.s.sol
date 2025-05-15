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


        uint256 ts = block.timestamp;


        verifier = new Groth16Verifier();
        voting = new Voting(address(verifier),ts+ts+ts,ts+ts+ts+ts);

        uint[2]  memory proofA =   [
        18954890798634224672748108392556735181778098235071477442101985231162520158622,
        5708118927076335960678206975902930031409523924089296379351094941737971853380
        ];    
            uint[2][2] memory proofB = [
        [2544285144128391970549401066802592834434653908835939200437408076668217852248, 18736488897089545574228196262818807291491391081397917053255163493104914571566],
        [20545688806147369478057416447713332573564362010727169535337799670749950574924, 7837928695099363029332175233078958187197469324998574093028231678892569889523]
    ];

        uint[2]  memory proofC =    [
            20222357992127944365972103554894159266234883276237604363568433352344516394799,
            2520475072948863462518606703662244566752048510716848923297159123121810266764
        ];
        uint[4]  memory pubSignals= [
            6522684365315333623294164708767841142998686809353049299512500329725983764769,
            17703948593860456708,
            1,
            0];

        voting.submitVote(proofA, proofB, proofC, pubSignals);

        vm.stopBroadcast();

        (uint256 result, uint256 r) = voting.getResults();
        console.log("Voting result:", result);
    }
}
// forge script VotingScript --rpc-url http://localhost:8545 --broadcast

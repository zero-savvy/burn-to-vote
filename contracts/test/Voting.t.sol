// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Voting} from "../src/Voting.sol";
import {Groth16Verifier} from "../src/verifier.sol";

contract VotingTest is Test {
    Voting public voting;
    Groth16Verifier public verifier;

    function setUp() public {
        // Deploy the verifier contract
        verifier = new Groth16Verifier();

        // Set up timestamps for voting and tally deadlines
        uint256 votingTime = block.timestamp + 10000;
        uint256 tallyTime = block.timestamp + 20000;
        uint256 ceremeny_id = 14564730128827983570;
        uint256 mt = 16140668641613092893634150840665751551734694739321141751642452107309003732465;

        // Deploy the Voting contract
        voting = new Voting(address(verifier), votingTime, tallyTime, mt, ceremeny_id);
    }

    function testDeployment() public {
        // Check that the verifier address is set correctly
        assertEq(address(voting.verifier()), address(verifier), "Verifier address mismatch");

        // Check that the vote submission deadline is set correctly
        uint256 expectedVotingTime = block.timestamp + 10000;
        assertEq(voting.voteSubmissionDeadline(), expectedVotingTime, "Vote submission deadline mismatch");

        // Check that the tally deadline is set correctly
        uint256 expectedTallyTime = block.timestamp + 20000;
        assertEq(voting.tallyDeadline(), expectedTallyTime, "Tally deadline mismatch");
    }

    function testVoteSubmissionDeadline() public {
        // Ensure the vote submission deadline is in the future
        uint256 deadline = voting.voteSubmissionDeadline();
        assertTrue(deadline > block.timestamp, "Vote submission deadline is not in the future");
    }

    function testTallyDeadline() public {
        // Ensure the tally deadline is after the vote submission deadline
        uint256 submissionDeadline = voting.voteSubmissionDeadline();
        uint256 tallyDeadline = voting.tallyDeadline();
        assertTrue(tallyDeadline > submissionDeadline, "Tally deadline is not after submission deadline");
    }

    function testVote() public {
        uint256[2] memory proofA = [
            18954890798634224672748108392556735181778098235071477442101985231162520158622,
            5708118927076335960678206975902930031409523924089296379351094941737971853380
        ];
        uint256[2][2] memory proofB = [
            [
                2544285144128391970549401066802592834434653908835939200437408076668217852248,
                18736488897089545574228196262818807291491391081397917053255163493104914571566
            ],
            [
                20545688806147369478057416447713332573564362010727169535337799670749950574924,
                7837928695099363029332175233078958187197469324998574093028231678892569889523
            ]
        ];
        // 292172
        uint256[2] memory proofC = [
            20222357992127944365972103554894159266234883276237604363568433352344516394799,
            2520475072948863462518606703662244566752048510716848923297159123121810266764
        ];
        uint256[5] memory pubSignals =
        [
            10415447804295826390464350432112598950919412374903053240204677308804537661856,
            14564730128827983570,
            1,
            0,
            16140668641613092893634150840665751551734694739321141751642452107309003732465
        ];

        voting.submitVote(proofA, proofB, proofC, pubSignals);
    }
}

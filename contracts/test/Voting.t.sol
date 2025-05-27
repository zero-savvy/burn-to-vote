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
        uint256 ceremeny_id = 16325618567567054338;
        uint256 mt = 6279442489816579343175600576641714715845361010123760250696645575814262324581;
        uint state_root = 9133689217370487228376476215699836963181592635914481284078419964281904630813;

        // Deploy the Voting contract
        voting = new Voting(address(verifier), votingTime, tallyTime, mt, ceremeny_id, state_root);
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
            11461911343166044330456695269059929792395400910531367637149148476455041492912,
            19886538862381079473207778911105727625315370822735581170211603430406046448838
        ];
        uint256[2][2] memory proofB = [
            [
                16250475503698907417494918297185825840713986646379737651665858753061739070827,
                2863378482915918189823727888280946879088275398006714090657506748037047959922
            ],
            [
                3281205606628297879259319685697544614446390832627250030433692892527431156623,
                12345284791679238322161188376002935890147679113261980361506659523949366149066
            ]
        ];
        // 292172
        uint256[2] memory proofC = [
            16388857710257556035315260696986375113607619359587989416845442029898498201229,
            18445830738733662355634800715146197283294980453956291346982500933921629929172
        ];
        uint256[6] memory pubSignals =
        [
            9133689217370487228376476215699836963181592635914481284078419964281904630813,
            1142708857887860899307168100545612092579662650614529027477380939122736649282,
            16325618567567054338,
            1,
            0,
            6279442489816579343175600576641714715845361010123760250696645575814262324581
        ];

        voting.submitVote(proofA, proofB, proofC, pubSignals);
    }

    function testTally() public {
        uint256[2] memory proofA = [
            11461911343166044330456695269059929792395400910531367637149148476455041492912,
            19886538862381079473207778911105727625315370822735581170211603430406046448838
        ];
        uint256[2][2] memory proofB = [
            [
                16250475503698907417494918297185825840713986646379737651665858753061739070827,
                2863378482915918189823727888280946879088275398006714090657506748037047959922
            ],
            [
                3281205606628297879259319685697544614446390832627250030433692892527431156623,
                12345284791679238322161188376002935890147679113261980361506659523949366149066
            ]
        ];
        // 292172
        uint256[2] memory proofC = [
            16388857710257556035315260696986375113607619359587989416845442029898498201229,
            18445830738733662355634800715146197283294980453956291346982500933921629929172
        ];
        uint256[6] memory pubSignals =
        [
            9133689217370487228376476215699836963181592635914481284078419964281904630813,
            1142708857887860899307168100545612092579662650614529027477380939122736649282,
            16325618567567054338,
            1,
            0,
            6279442489816579343175600576641714715845361010123760250696645575814262324581
        ];

        voting.submitVote(proofA, proofB, proofC, pubSignals);

        vm.warp(block.timestamp + 20000);

        voting.tallyVotes();
        voting.getResults();
    }

        function testRevote() public {
        uint256[2] memory proofA = [
            11461911343166044330456695269059929792395400910531367637149148476455041492912,
            19886538862381079473207778911105727625315370822735581170211603430406046448838
        ];
        uint256[2][2] memory proofB = [
            [
                16250475503698907417494918297185825840713986646379737651665858753061739070827,
                2863378482915918189823727888280946879088275398006714090657506748037047959922
            ],
            [
                3281205606628297879259319685697544614446390832627250030433692892527431156623,
                12345284791679238322161188376002935890147679113261980361506659523949366149066
            ]
        ];
        // 292172
        uint256[2] memory proofC = [
            16388857710257556035315260696986375113607619359587989416845442029898498201229,
            18445830738733662355634800715146197283294980453956291346982500933921629929172
        ];
        uint256[6] memory pubSignals =
        [
            9133689217370487228376476215699836963181592635914481284078419964281904630813,
            1142708857887860899307168100545612092579662650614529027477380939122736649282,
            16325618567567054338,
            1,
            0,
            6279442489816579343175600576641714715845361010123760250696645575814262324581
        ];

        voting.submitVote(proofA, proofB, proofC, pubSignals);
        voting.submitRevote(proofA, proofB, proofC, pubSignals, proofA, proofB, proofC, pubSignals);
    }
}

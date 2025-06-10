// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {VotingFactory} from "../src/VotingFactory.sol";
import {Voting} from "../src/Voting.sol";
import {Groth16Verifier} from "../src/verifier.sol";
import {VotingPeriodEnded, InvalidVote, NullifierAlreadyUsed, RevotingNotAllowed, NullifierMismatch, InvalidRevoteValue} from "../src/Errors.sol";


contract VotingTest is Test {
    VotingFactory public votingFactory;
    Voting public voting;
    Groth16Verifier public verifier;

    uint256 submissionDeadline = block.timestamp + 10000;
    uint256 tallyDeadline = block.timestamp + 20000;
    uint256 ceremonyId = 7496824597605666358;
    uint256 merkleRoot = 2460315761077221111371079137112756400365219279656913426802070198701997759860;
    uint256 stateRoot = 16580658621263755616398168707812265815499667385403522275332483037094862039449;
    bytes32 salt = keccak256(abi.encodePacked(ceremonyId));

    event VoteSubmitted(address indexed voter, uint256 nullifier, uint256 vote);

    uint256[2] proofA = [
        15735234741036841031785444279490969598560196843641202116530397007303416185676,
        18150691990194341536057655269208744335697765123953767493612463362207859936053
    ];
    uint256[2][2] proofB = [
        [
            13266005699425396071609737237474717092791047403386583977505335368316493653834,
            10496723520671847229028022026343553242445142007883043464366930756731799554837
        ],
        [
            3995591568799800093966694878544427925521265551962048650602930242215399447150,
            7737754626396588929108482656761721100100553711177325925043050569539144715458
        ]
    ];
    uint256[2] proofC = [
        1002095530778988323297856129135406269799571698438808857309929901200889493124,
        13059467832921364130052777072151339381043283688520739469674524537959352978453
    ];
    uint256[6] pubSignals = [
        16580658621263755616398168707812265815499667385403522275332483037094862039449,
        9891682204978241203411276824529809676739256861554734859603992321540123681204,
        7496824597605666358,
        1,
        0,
        2460315761077221111371079137112756400365219279656913426802070198701997759860
    ];

    function setUp() public {
        verifier = new Groth16Verifier();
        votingFactory = new VotingFactory();
        address votingAddress = votingFactory.deployVotingContract(
            salt,
            VotingFactory.CeremonyType.Binary,
            address(verifier),
            submissionDeadline,
            tallyDeadline,
            merkleRoot,
            ceremonyId,
            stateRoot
        );
        voting = Voting(votingAddress);
    }

    function testVote() public {
        vm.expectEmit(true, true, true, true);
        emit VoteSubmitted(address(this), pubSignals[1], pubSignals[3]);
        voting.submitVote(proofA, proofB, proofC, pubSignals);
    }

    function testDoubleVote() public {
        voting.submitVote(proofA, proofB, proofC, pubSignals);
        vm.expectRevert(abi.encodeWithSelector(NullifierAlreadyUsed.selector, pubSignals[1]));
        voting.submitVote(proofA, proofB, proofC, pubSignals);
    }

    function testVotingTime() public {
        vm.warp(submissionDeadline + 1);
        vm.expectRevert(abi.encodeWithSelector(VotingPeriodEnded.selector, submissionDeadline, submissionDeadline + 1));
        voting.submitVote(proofA, proofB, proofC, pubSignals);
    }

    function testInvalidVote() public {
        uint256[6] memory invalidVoteSignals = pubSignals;
        invalidVoteSignals[3] = 2;
        vm.expectRevert(abi.encodeWithSelector(InvalidVote.selector, 2));
        voting.submitVote(proofA, proofB, proofC, invalidVoteSignals);
    }

    function testRevoteNotAllowed() public {
        uint256[6] memory revoteSignals = pubSignals;
        revoteSignals[4] = 1;
        vm.expectRevert(RevotingNotAllowed.selector);
        voting.submitVote(proofA, proofB, proofC, revoteSignals);
    }

    // function testRevote() public {
    //     voting.submitVote(proofA, proofB, proofC, pubSignals);
    //     uint256[6] memory revoteSignals = pubSignals;
    //     revoteSignals[3] = 0;
    //     revoteSignals[4] = 1;
    //     vm.expectEmit(true, true, true, true);
    //     emit VoteSubmitted(address(this), revoteSignals[1], revoteSignals[3]);
    //     voting.submitRevote(
    //         proofA, proofB, proofC, pubSignals,
    //         proofA, proofB, proofC, revoteSignals
    //     );
    // }

    function testRevoteInvalidNullifier() public {
        voting.submitVote(proofA, proofB, proofC, pubSignals);
        uint256[6] memory revoteSignals = pubSignals;
        revoteSignals[1] = 123;
        revoteSignals[3] = 0;
        revoteSignals[4] = 1;
        vm.expectRevert(abi.encodeWithSelector(NullifierMismatch.selector, pubSignals[1], 123));
        voting.submitRevote(
            proofA, proofB, proofC, pubSignals,
            proofA, proofB, proofC, revoteSignals
        );
    }

    function testRevoteSameVote() public {
        voting.submitVote(proofA, proofB, proofC, pubSignals);
        uint256[6] memory revoteSignals = pubSignals;
        revoteSignals[4] = 1;
        vm.expectRevert(InvalidRevoteValue.selector);
        voting.submitRevote(
            proofA, proofB, proofC, pubSignals,
            proofA, proofB, proofC, revoteSignals
        );
    }

    function testRevoteAfterDeadline() public {
        voting.submitVote(proofA, proofB, proofC, pubSignals);
        uint256[6] memory revoteSignals = pubSignals;
        revoteSignals[3] = 0;
        revoteSignals[4] = 1;
        vm.warp(submissionDeadline + 1);
        vm.expectRevert(abi.encodeWithSelector(VotingPeriodEnded.selector, submissionDeadline, submissionDeadline + 1));
        voting.submitRevote(
            proofA, proofB, proofC, pubSignals,
            proofA, proofB, proofC, revoteSignals
        );
    }

    function testRevoteWithoutInitialVote() public {
        uint256[6] memory revoteSignals = pubSignals;
        revoteSignals[3] = 0;
        revoteSignals[4] = 1;
        vm.expectRevert(abi.encodeWithSelector(NullifierAlreadyUsed.selector, pubSignals[1]));
        voting.submitRevote(
            proofA, proofB, proofC, pubSignals,
            proofA, proofB, proofC, revoteSignals
        );
    }
}


// TODO: add edge case tests

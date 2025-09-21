// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Factory} from "../src/Factory.sol";
import {Voting} from "../src/Voting.sol";
import {Groth16Verifier} from "../src/verifier.sol";
import {
    SubmissionPeriodEnded,
    InvalidVote,
    NullifierAlreadyUsed,
    RevotingNotAllowed,
    NullifierMismatch,
    InvalidRevoteValue
} from "../src/Errors.sol";

contract VotingTest is Test {
    Factory public factory;
    Voting public voting;
    Groth16Verifier public verifier;

    uint256 submissionDeadline = block.timestamp + 10000;
    uint256 tallyDeadline = block.timestamp + 20000;
    uint256 ceremonyId = 10000943969678960357;
    uint256 merkleRoot = 16438268736072512344141018254739701154094966687964638349191951581975302451728;
    uint256 stateRoot = 11031234271448505540032213211604743113794925574184185056246879085167736838935;
    bytes32 salt = keccak256(abi.encodePacked(ceremonyId));

    event VoteSubmitted(address indexed voter, uint256 nullifier, uint256 vote);

    uint256[2] proofA = [
        15965792215941395289952765997280110026061925108119605184003763368104497472345,
        7488613381170103776807550210253897966123645872170826777511516423381612077512
    ];
    uint256[2][2] proofB = [
        [
            14413901228087177783266521320822499174518410694075970649212879841022467092773,
            2099347525664573321853165641721371496184841196850985861209036755555137970671
        ],
        [
            16402948553328916506107961831631870093100917218767140807029998903269293406007,
            18438477925854996461456198315450896132608589561523719353256246173298461308978
        ]
    ];
    uint256[2] proofC = [
        12156085179901873914439343160911937402265356584236388337137907871813560406772,
        7665277494946264498154874495107458343942413456699728828450206219497656404407
    ];
    uint256[6] pubSignals = [
        11031234271448505540032213211604743113794925574184185056246879085167736838935,
        7865555154734265330107895498919312998651559920337636186800573049344025728422,
        10000943969678960357,
        1,
        0,
        16438268736072512344141018254739701154094966687964638349191951581975302451728
    ];

    uint256[2] revoteProofA = [
        20290151011281465599700283300023276933524216116599835085057612776008865457450,
        15264568993652883558797151797120711118530682598148575554928947260448446065333
    ];
    uint256[2][2] revoteProofB = [
        [
            16516404403054057478896328818123976905791884836314623483663563638048442794842,
            7348388964977453758709841391688783731930748315550047853477458015117361096268
        ],
        [
            12664766092832948557838887778752375829552132518384103097738374322783530394834,
            12401058616006635027164584149439106542108100473636594241463235158099744439753
        ]
    ];
    uint256[2] revoteProofC = [
        4002617130189427431558412067419678838351695831697724549257109404266010367507,
        21422883597551817849682437478108040905635239415833191613127012705245229505149
    ];
    uint256[6] revotePubSignals = [
        11031234271448505540032213211604743113794925574184185056246879085167736838935,
        7865555154734265330107895498919312998651559920337636186800573049344025728422,
        10000943969678960357,
        1,
        1,
        16438268736072512344141018254739701154094966687964638349191951581975302451728
    ];

    function setUp() public {
        verifier = new Groth16Verifier();
        factory = new Factory();
        address votingAddress = votingFactory.deployVotingContract(
            salt,
            factory.CeremonyType.Binary,
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
        vm.expectRevert(
            abi.encodeWithSelector(SubmissionPeriodEnded.selector, submissionDeadline, submissionDeadline + 1)
        );
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

    function testRevote() public {
        voting.submitVote(proofA, proofB, proofC, pubSignals);
        vm.expectEmit(true, true, true, true);
        emit VoteSubmitted(address(this), pubSignals[1], pubSignals[3]);
        voting.submitRevote(
            proofA, proofB, proofC, pubSignals, revoteProofA, revoteProofB, revoteProofC, revotePubSignals
        );
    }

    function testRevoteInvalidNullifier() public {
        voting.submitVote(proofA, proofB, proofC, pubSignals);
        revotePubSignals[1] = 123;
        revotePubSignals[3] = 0;
        revotePubSignals[4] = 1;
        vm.expectRevert(abi.encodeWithSelector(NullifierMismatch.selector, pubSignals[1], 123));
        voting.submitRevote(
            proofA, proofB, proofC, pubSignals, revoteProofA, revoteProofB, revoteProofC, revotePubSignals
        );
    }

    function testRevoteAfterDeadline() public {
        voting.submitVote(proofA, proofB, proofC, pubSignals);
        revotePubSignals[3] = 0;
        revotePubSignals[4] = 1;
        vm.warp(submissionDeadline + 1);
        vm.expectRevert(
            abi.encodeWithSelector(SubmissionPeriodEnded.selector, submissionDeadline, submissionDeadline + 1)
        );
        voting.submitRevote(
            proofA, proofB, proofC, pubSignals, revoteProofA, revoteProofB, revoteProofC, revotePubSignals
        );
    }

    function testRevoteWithoutInitialVote() public {
        vm.expectRevert(abi.encodeWithSelector(NullifierAlreadyUsed.selector, pubSignals[1]));
        voting.submitRevote(
            proofA, proofB, proofC, pubSignals, revoteProofA, revoteProofB, revoteProofC, revotePubSignals
        );
    }
}

// TODO: add edge case tests

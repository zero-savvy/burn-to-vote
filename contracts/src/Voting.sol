// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./verifier.sol"; 


contract Voting {
    Groth16Verifier public verifier;

    mapping(bytes32 => bool) public usedNullifiers;

    uint256 public voteSubmissionDeadline;
    uint256 public tallyDeadline;

    uint128 private yesVotes;
    uint128 private noVotes;

    bool public tallyCompleted;

    event VoteSubmitted(address indexed voter, bytes32 nullifier, uint8 vote);
    event VotingResults(uint256 yesVotes, uint256 noVotes, bool passed);
    event GasUsed(uint256 gasUsed);

    constructor(address _verifier, uint256 _submissionDeadline, uint256 _tallyDeadline) {
        require(_submissionDeadline < _tallyDeadline, "Submission deadline must be before tally deadline");
        verifier = Groth16Verifier(_verifier);
        voteSubmissionDeadline = _submissionDeadline;
        tallyDeadline = _tallyDeadline;
    }

    function submitVote(
        uint[2] calldata proofA,      
        uint[2][2] calldata proofB,  
        uint[2] calldata proofC,     
        uint[2] calldata pubSignals,  
        bytes32 nullifier,
        uint8 vote
        ) external {
            require(block.timestamp < voteSubmissionDeadline, "Voting period has ended");
            require(vote == 0 || vote == 1, "Invalid vote");
            require(!usedNullifiers[nullifier], "Nullifier already used");

            bool proofIsValid = verifier.verifyProof(proofA, proofB, proofC, pubSignals);
            require(proofIsValid, "Invalid Merkle proof");

            usedNullifiers[nullifier] = true;

            if (vote == 1) {
                yesVotes++;
            } else {
                noVotes++;
            }

            emit VoteSubmitted(msg.sender, nullifier, vote);
        }

    function tallyVotes() external {
        require(block.timestamp >= tallyDeadline, "Tallying is not yet allowed");
        require(!tallyCompleted, "Tally has already been completed");

        tallyCompleted = true;
        bool passed = yesVotes > noVotes;

        emit VotingResults(yesVotes, noVotes, passed);
    }

    function getResults() external view returns (uint256, uint256) {
        require(tallyCompleted, "Results not available yet");
        return (yesVotes, noVotes);
    }
}

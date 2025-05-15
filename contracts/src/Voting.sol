// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./verifier.sol"; 


contract Voting {
    Groth16Verifier public verifier;

    mapping(uint => uint8) public usedNullifiers;

    uint256 public voteSubmissionDeadline;
    uint256 public tallyDeadline;

    uint128 private yesVotes;
    uint128 private noVotes;

    bool public tallyCompleted;

    event VoteSubmitted(address indexed voter, uint nullifier, uint vote);
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
        uint[4] calldata pubSignals
        ) external {
            require(block.timestamp < voteSubmissionDeadline, "Voting period has ended");
            require(pubSignals[3] == 0 || pubSignals[3] == 1, "Invalid vote");
            require(usedNullifiers[pubSignals[2]] < 2, "Nullifier already used");

            bool proofIsValid = verifier.verifyProof(proofA, proofB, proofC, pubSignals);
            require(proofIsValid, "Invalid Merkle proof");

            usedNullifiers[pubSignals[2]] += 1;

            if (pubSignals[3] == 1) {
                yesVotes++;
            } else {
                noVotes++;
            }

            emit VoteSubmitted(msg.sender, pubSignals[2], pubSignals[3]);
        }

    function submitRevote(
        uint[2] calldata old_proofA,      
        uint[2][2] calldata old_proofB,  
        uint[2] calldata old_proofC,     
        uint[4] calldata old_pubSignals,
        uint[2] calldata new_proofA,      
        uint[2][2] calldata new_proofB,  
        uint[2] calldata new_proofC,     
        uint[4] calldata new_pubSignals
        ) external {
            require(block.timestamp < voteSubmissionDeadline, "Voting period has ended");
            require(old_pubSignals[3] == 0 || old_pubSignals[3] == 1, "Invalid vote");
            require(new_pubSignals[4] == 1, "Invalid revote proof");
            require(usedNullifiers[old_pubSignals[2]] < 2, "Nullifier already used");

            bool oldProofIsValid = verifier.verifyProof(old_proofA, old_proofB, old_proofC, old_pubSignals);
            require(oldProofIsValid, "Invalid old voting proof");

            bool proofIsValid = verifier.verifyProof(new_proofA, new_proofB, new_proofC, new_pubSignals);
            require(proofIsValid, "Invalid revoting proof");

            

            usedNullifiers[old_pubSignals[2]] += 1;

            if (old_pubSignals[3] == 1) {
                yesVotes++;
            } else {
                noVotes++;
            }

            emit VoteSubmitted(msg.sender, new_pubSignals[2], new_pubSignals[3]);
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

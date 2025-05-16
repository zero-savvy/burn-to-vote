// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./verifier.sol";
import "./Errors.sol";

contract Voting {
    Groth16Verifier public verifier;

    uint256 public ceremony_id;
    uint256 public merkle_root;
    uint256 public voteSubmissionDeadline;
    uint256 public tallyDeadline;

    uint128 private yesVotes;
    uint128 private noVotes;

    bool public tallyCompleted;

    mapping(uint256 => uint8) public usedNullifiers;

    event VoteSubmitted(address indexed voter, uint256 nullifier, uint256 vote);
    event VotingResults(uint256 yesVotes, uint256 noVotes, bool passed);
    event GasUsed(uint256 gasUsed);

    constructor(
        address _verifier,
        uint256 _submissionDeadline,
        uint256 _tallyDeadline,
        uint256 _merkle_root,
        uint256 _ceremony_id
    ) {
        require(_submissionDeadline < _tallyDeadline, "Submission deadline must be before tally deadline");
        verifier = Groth16Verifier(_verifier);
        voteSubmissionDeadline = _submissionDeadline;
        tallyDeadline = _tallyDeadline;
        merkle_root = _merkle_root;
        ceremony_id = _ceremony_id;
    }

    function submitVote(
        uint256[2] calldata proofA,
        uint256[2][2] calldata proofB,
        uint256[2] calldata proofC,
        uint256[5] calldata pubSignals
    ) external {
        if (block.timestamp > voteSubmissionDeadline) revert VotingPeriodEnded(voteSubmissionDeadline, block.timestamp);
        if (!(pubSignals[2] == 0 || pubSignals[2] == 1)) revert InvalidVote(pubSignals[2]);
        if (usedNullifiers[pubSignals[0]] != 0) revert NullifierAlreadyUsed(pubSignals[0]);
        if (pubSignals[3] == 1) revert RevotingNotAllowed();
        if (pubSignals[4] != merkle_root) revert InvalidMerkleRoot(pubSignals[4], merkle_root);
        if (pubSignals[1] != ceremony_id) revert InvalidCeremonyId(pubSignals[1], ceremony_id);

        bool proofIsValid = verifier.verifyProof(proofA, proofB, proofC, pubSignals);
        if (!proofIsValid) revert InvalidProof();

        usedNullifiers[pubSignals[0]] += 1;

        if (pubSignals[2] == 1) {
            yesVotes++;
        } else {
            noVotes++;
        }

        emit VoteSubmitted(msg.sender, pubSignals[0], pubSignals[2]);
    }

    function submitRevote(
        uint256[2] calldata old_proofA,
        uint256[2][2] calldata old_proofB,
        uint256[2] calldata old_proofC,
        uint256[4] calldata old_pubSignals,
        uint256[2] calldata new_proofA,
        uint256[2][2] calldata new_proofB,
        uint256[2] calldata new_proofC,
        uint256[4] calldata new_pubSignals
    ) external {
        if (block.timestamp > voteSubmissionDeadline) revert VotingPeriodEnded(voteSubmissionDeadline, block.timestamp);

        if (old_pubSignals[0] != new_pubSignals[0]) revert NullifierMismatch(old_pubSignals[0], new_pubSignals[0]);

        if (!(old_pubSignals[2] == 0 || old_pubSignals[2] == 1)) revert InvalidVote(old_pubSignals[2]);
        if (old_pubSignals[3] == 1) revert RevotingNotAllowed();
        if (old_pubSignals[4] != merkle_root) revert InvalidMerkleRoot(old_pubSignals[4], merkle_root);
        if (old_pubSignals[1] != ceremony_id) revert InvalidCeremonyId(old_pubSignals[1], ceremony_id);
        bool oldProofIsValid = verifier.verifyProof(old_proofA, old_proofB, old_proofC, old_pubSignals);
        if (!oldProofIsValid) revert InvalidProof();

        if (!(new_pubSignals[2] == 0 || new_pubSignals[2] == 1)) revert InvalidVote(new_pubSignals[2]);
        if (new_pubSignals[2] == old_pubSignals[2]) revert InvalidRevoteValue();
        if (usedNullifiers[new_pubSignals[0]] != 1) revert NullifierAlreadyUsed(new_pubSignals[0]);
        if (new_pubSignals[3] == 0) revert RevotingNotAllowed();
        if (new_pubSignals[4] != merkle_root) revert InvalidMerkleRoot(new_pubSignals[4], merkle_root);
        if (new_pubSignals[1] != ceremony_id) revert InvalidCeremonyId(new_pubSignals[1], ceremony_id);
        bool proofIsValid = verifier.verifyProof(new_proofA, new_proofB, new_proofC, new_pubSignals);
        if (!proofIsValid) revert InvalidProof();

        usedNullifiers[old_pubSignals[2]] += 1;

        if (old_pubSignals[3] == 1 && new_pubSignals[3] == 0) {
            yesVotes--;
            noVotes++;
        } else if (old_pubSignals[3] == 0 && new_pubSignals[3] == 1) {
            noVotes--;
            yesVotes++;
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

// proving time
// gas estimatuin
// compare to others in large scale

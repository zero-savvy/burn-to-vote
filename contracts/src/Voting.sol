// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./verifier.sol";
import "./Errors.sol";
// add voting deadline

/**
 * @title Voting
 * @dev A contract for anonymous voting using zero-knowledge proofs (zk-SNARKs).
 * This contract implements a binary voting system where voters can cast votes anonymously
 * by providing valid zero-knowledge proofs. The system supports vote submission and tallying,
 * with built-in deadlines for both operations.
 */
contract Voting {
    Groth16Verifier public verifier;

    uint256 public ceremony_id;
    uint256 public merkle_root;
    uint256 public voteSubmissionDeadline;
    uint256 public tallyDeadline;
    uint256 public state_root;
    uint128 private yesVotes;
    uint128 private noVotes;

    bool public tallyCompleted = false;

    bool private initialized = false;

    /// @notice Mapping to track used nullifiers to prevent double voting
    /// @dev Maps nullifier hash to usage count (0 if not, 1 if used, 2 if revoted)
    mapping(uint256 => uint8) public usedNullifiers;

    event VoteSubmitted(address indexed voter, uint256 nullifier, uint256 vote);

    event VotingResults(uint256 yesVotes, uint256 noVotes, bool passed);

    /**
     * @dev Initializes the voting contract with required parameters
     * @param _verifier Address of the Groth16 verifier contract
     * @param _submissionDeadline Timestamp after which votes cannot be submitted
     * @param _tallyDeadline Timestamp after which votes can be tallied
     * @param _merkle_root Merkle root of the allowed voters tree
     * @param _ceremony_id Unique identifier for the voting ceremony
     * @param _state_root State root of the last voting block
     */
    function initialize(
        address _verifier,
        uint256 _submissionDeadline,
        uint256 _tallyDeadline,
        uint256 _merkle_root,
        uint256 _ceremony_id,
        uint256 _state_root
    ) external {
        require(!initialized, "Already initialized");
        require(_submissionDeadline < _tallyDeadline, "Submission deadline must be before tally deadline");

        verifier = Groth16Verifier(_verifier);
        voteSubmissionDeadline = _submissionDeadline;
        tallyDeadline = _tallyDeadline;
        merkle_root = _merkle_root;
        ceremony_id = _ceremony_id;
        state_root = _state_root;

        initialized = true;
    }

    /**
     * @dev Submits a vote with a zero-knowledge proof
     * @param proofA, proofB, proofC Groth16 proof
     * @param pubSignals Public signals for the proof verification
     * @notice The proof must verify that:
     *         1. The voter burn address is in the mpt
     *         2. The nullifier hasn't been used before
     *         3. The vote is valid (0 or 1)
     *         4. The ceremony ID matches
     */
    function submitVote(
        uint256[2] calldata proofA,
        uint256[2][2] calldata proofB,
        uint256[2] calldata proofC,
        uint256[6] calldata pubSignals
    ) external {
        if (block.timestamp > voteSubmissionDeadline) {
            revert SubmissionPeriodEnded(voteSubmissionDeadline, block.timestamp);
        }
        if (!(pubSignals[3] == 0 || pubSignals[3] == 1)) revert InvalidVote(pubSignals[3]);
        if (usedNullifiers[pubSignals[1]] != 0) revert NullifierAlreadyUsed(pubSignals[1]);
        if (pubSignals[4] == 1) revert RevotingNotAllowed();

        bool proofIsValid = verifier.verifyProof(
            proofA, proofB, proofC, [state_root, pubSignals[1], ceremony_id, pubSignals[3], pubSignals[4], merkle_root]
        );
        if (!proofIsValid) revert InvalidProof();

        usedNullifiers[pubSignals[1]] += 1;

        if (pubSignals[3] == 1) {
            yesVotes++;
        } else {
            noVotes++;
        }

        emit VoteSubmitted(msg.sender, pubSignals[1], pubSignals[3]);
    }

    /**
     * @dev Submits a vote change with zero-knowledge proofs
     * @param old_proofA, proofB, proofC, the old vote proof
     * @param old_pubSignals Public signals for the old vote proof
     * @param new_proofA, proofB, proofC, the new vote proof
     * @param new_pubSignals Public signals for the new vote proof
     * @notice Allows voters to change their vote by providing proofs for both old and new votes
     */
    function submitRevote(
        uint256[2] calldata old_proofA,
        uint256[2][2] calldata old_proofB,
        uint256[2] calldata old_proofC,
        uint256[6] calldata old_pubSignals,
        uint256[2] calldata new_proofA,
        uint256[2][2] calldata new_proofB,
        uint256[2] calldata new_proofC,
        uint256[6] calldata new_pubSignals
    ) external {
        if (block.timestamp > voteSubmissionDeadline) {
            revert SubmissionPeriodEnded(voteSubmissionDeadline, block.timestamp);
        }

        if (old_pubSignals[1] != new_pubSignals[1]) revert NullifierMismatch(old_pubSignals[1], new_pubSignals[1]);

        if (!(old_pubSignals[3] == 0 || old_pubSignals[3] == 1)) revert InvalidVote(old_pubSignals[3]);
        if (old_pubSignals[4] == 1) revert RevotingNotAllowed();
        bool oldProofIsValid = verifier.verifyProof(
            old_proofA,
            old_proofB,
            old_proofC,
            [state_root, old_pubSignals[1], ceremony_id, old_pubSignals[3], old_pubSignals[4], merkle_root]
        );

        if (!oldProofIsValid) revert InvalidProof();

        if (!(new_pubSignals[3] == 0 || new_pubSignals[3] == 1)) revert InvalidVote(new_pubSignals[3]);
        if (new_pubSignals[3] != old_pubSignals[3]) revert InvalidRevoteValue();
        if (usedNullifiers[new_pubSignals[1]] != 1) revert NullifierAlreadyUsed(new_pubSignals[1]);
        if (new_pubSignals[4] == 0) revert RevotingNotAllowed();
        bool proofIsValid = verifier.verifyProof(
            new_proofA,
            new_proofB,
            new_proofC,
            [state_root, new_pubSignals[1], ceremony_id, new_pubSignals[3], new_pubSignals[4], merkle_root]
        );
        if (!proofIsValid) revert InvalidProof();

        usedNullifiers[old_pubSignals[1]] += 1;

        if (old_pubSignals[3] == 1) {
            yesVotes--;
            noVotes++;
        } else if (old_pubSignals[3] == 0) {
            noVotes--;
            yesVotes++;
        }

        emit VoteSubmitted(msg.sender, new_pubSignals[1], new_pubSignals[3]);
    }

    /**
     * @dev Tallies the votes and emits the results
     * @notice Can only be called after the tally deadline
     * @notice Can only be called once
     * @notice Emits VotingResults event with final vote counts
     */
    function tallyVotes() external {
        if (block.timestamp <= tallyDeadline) revert TallyNotAllowd();
        require(!tallyCompleted, "Tally has already been completed");

        tallyCompleted = true;
        bool passed = yesVotes > noVotes;

        emit VotingResults(yesVotes, noVotes, passed);
    }

    /**
     * @dev Returns the current vote results
     * @return yesVotes_ Number of yes votes
     * @return noVotes_ Number of no votes
     * @return passed Whether the vote passed (yes > no)
     */
    function getResults() external view returns (uint256 yesVotes_, uint256 noVotes_, bool passed) {
        require(tallyCompleted, "Results not available yet");
        return (yesVotes, noVotes, yesVotes > noVotes);
    }
}

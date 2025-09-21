// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./verifier.sol";
import "./Errors.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuardTransient.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/**
 * @title Auction
 * @dev A contract for anonymous bidding using zero-knowledge proofs (zk-SNARKs).
 * This contract implements an auction system where bidders can submit bids anonymously
 * by providing valid zero-knowledge proofs. The system supports bid submission with collateral,
 * winner determination, and collateral withdrawal for non-winners.
 */
contract Auction is ReentrancyGuardTransient {
    Groth16Verifier public verifier;

    uint256 public merkleRoot;
    uint256 public stateRoot;
    uint256 public ceremonyId;

    uint256 public biddingDeadline;
    uint256 public bidSubmissionDeadline;
    uint256 public resultDeadline;

    uint256[] winningBids;
    address[] winners;

    event BidSubmitted(address indexed bidder, uint256 bid);

    bool private initialized = false;
    mapping(uint256 => bool) public usedNullifiers;
    mapping(address => uint256) public collateral;

    /**
     * @dev Initializes the auction contract with required parameters
     * @param _verifier Address of the Groth16 verifier contract
     * @param _biddingDeadline Timestamp after which bidding period ends
     * @param _submissionDeadline Timestamp after which bid submission period ends
     * @param _resultDeadline Timestamp after which results can be revealed
     * @param _merkleRoot Merkle root of the allowed bidders tree
     * @param _ceremonyId Unique identifier for the auction ceremony
     * @param _stateRoot State root of the last auction block
     * @param _maxWinners Maximum number of winners in this auction
     * @notice Can only be called once per contract instance
     */
    function initialize(
        address _verifier,
        uint256 _biddingDeadline,
        uint256 _submissionDeadline,
        uint256 _resultDeadline,
        uint256 _merkleRoot,
        uint256 _ceremonyId,
        uint256 _stateRoot,
        uint256 _maxWinners
    ) external {
        verifier = Groth16Verifier(_verifier);
        merkleRoot = _merkleRoot;
        stateRoot = _stateRoot;
        biddingDeadline = _biddingDeadline;
        bidSubmissionDeadline = _submissionDeadline;
        resultDeadline = _resultDeadline;
        ceremonyId = _ceremonyId;
        winners = new address[](_maxWinners);
        winningBids = new uint256[](_maxWinners);
        initialized = true;
    }

    /**
     * @dev Submits a bid with a zero-knowledge proof
     * @param proofA, proofB, proofC Groth16 proof components
     * @param pubSignals Public signals for the proof verification
     * @notice The proof must verify that:
     *         1. The bidder burn address is in the mpt
     *         2. The nullifier hasn't been used before
     *         3. The bid amount matches the sent ETH
     *         4. The ceremony ID matches
     * @notice Requires ETH payment equal to the bid amount as collateral
     * @notice Can only be called during the bid submission period
     */
    function submitBid(
        uint256[2] calldata proofA,
        uint256[2][2] calldata proofB,
        uint256[2] calldata proofC,
        uint256[6] calldata pubSignals
    ) external payable {
        if (block.timestamp < biddingDeadline) revert CastingPeriodEnded(biddingDeadline, block.timestamp);
        if (block.timestamp > bidSubmissionDeadline) {
            revert SubmissionPeriodEnded(bidSubmissionDeadline, block.timestamp);
        }
        if (usedNullifiers[pubSignals[1]]) revert NullifierAlreadyUsed(pubSignals[1]);

        bool proofIsValid = verifier.verifyProof(
            proofA, proofB, proofC, [stateRoot, pubSignals[1], ceremonyId, pubSignals[3], pubSignals[4], merkleRoot]
        );
        if (!proofIsValid) revert InvalidProof();

        usedNullifiers[pubSignals[1]] = true;

        if (msg.value != pubSignals[3]) revert InvalidCollateral(msg.value, pubSignals[3]);

        checkAndInsertBid(pubSignals[3], msg.sender);

        collateral[msg.sender] = pubSignals[3];

        emit BidSubmitted(msg.sender, pubSignals[3]);
    }

    /**
     * @dev Checks if a bid qualifies as a winning bid and inserts it into the winners array
     * @param bid The bid amount to check
     * @param bidder The address of the bidder
     * @return bool True if the bid was inserted, false if it doesn't qualify
     * @notice Maintains the winners array sorted in descending order by bid amount
     * @notice Only keeps track of the top N winners (where N is maxWinners)
     */
    function checkAndInsertBid(uint256 bid, address bidder) internal returns (bool) {
        for (uint256 i = 0; i < winners.length; i++) {
            if (bid > winningBids[i]) {
                for (uint256 j = winners.length - 1; j > i; j--) {
                    winners[j] = winners[j - 1];
                    winningBids[j] = winningBids[j - 1];
                }
                winningBids[i] = bid;
                winners[i] = bidder;
                return true;
            }
        }
        return false;
    }

    /**
     * @dev Returns the auction results (winners and their bids)
     * @return address[] Array of winner addresses (sorted by bid amount, descending)
     * @return uint256[] Array of winning bid amounts (sorted in descending order)
     * @notice Can only be called after the result deadline
     * @notice Returns empty arrays if called before deadline
     */
    function results() external view returns (address[] memory, uint256[] memory) {
        if (block.timestamp < resultDeadline) revert TallyNotAllowd();
        return (winners, winningBids);
    }

    /**
     * @dev Allows non-winners to withdraw their collateral
     * @notice Can only be called after the result deadline
     * @notice Winners cannot withdraw their collateral (it serves as payment)
     * @notice Transfers the full collateral amount back to the caller
     */
    function withdrawCollateral() external {
        if (block.timestamp < resultDeadline) revert TallyNotAllowd();
        if (msg.sender == winners[0]) revert WinnerCannotWithdraw();
        payable(msg.sender).transfer(collateral[msg.sender]);
    }
}

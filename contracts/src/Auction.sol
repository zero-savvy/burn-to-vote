// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./verifier.sol";
import "./Errors.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

contract Auction {
    Groth16Verifier public verifier;

    uint256 merkleRoot;
    uint256 stateRoot;
    uint256 ceremonId;

    uint256 biddingDeadline;
    uint256 bidSubmissionDeadline;
    uint256 resultDealine;

    uint256[] winingBids;
    address[] winners;
    address biddingToken;


    event BidSubmitted(address indexed bidder, uint256 bid);
    bool private initialized = false;
    mapping(uint256 => bool) public usedNullifiers;


    function initialize(
        address _verifier,
        uint256 _biddingDealine,
        uint256 _submissionDeadline,
        uint256 _ResultDeadline,
        uint256 _merkleRoot,
        uint256 _ceremonyId,
        uint256 _stateRoot,
        address _biddingToken,
        uint256 _maxWinners
    ) external {
        verifier = Groth16Verifier(_verifier);
        merkleRoot = _merkleRoot;
        stateRoot = _stateRoot;
        biddingDeadline = _biddingDealine;
        bidSubmissionDeadline = _submissionDeadline;
        resultDealine = _ResultDeadline;
        ceremonId = _ceremonyId;
        biddingToken = _biddingToken;
        winners = new address[](_maxWinners);
        winingBids = new uint256[](_maxWinners);
        initialized = true;
    }

    function submitBid(
        uint256[2] calldata proofA,
        uint256[2][2] calldata proofB,
        uint256[2] calldata proofC,
        uint256[6] calldata pubSignals
    ) external payable returns (address) {
        if (block.timestamp < biddingDeadline) revert CastingPeriodEnded(biddingDeadline, block.timestamp);
        if (block.timestamp > bidSubmissionDeadline) {
            revert SubmissionPeriodEnded(bidSubmissionDeadline, block.timestamp);
        }
        if (usedNullifiers[pubSignals[1]]) revert NullifierAlreadyUsed(pubSignals[1]);
        bool proofIsValid = verifier.verifyProof(
            proofA, proofB, proofC, [stateRoot, pubSignals[1], ceremonId, pubSignals[3], pubSignals[4], merkleRoot]
        );
        if (!proofIsValid) revert InvalidProof();

        usedNullifiers[pubSignals[1]] = true;

        if (msg.value != pubSignals[3]) revert InvalidCollateral(msg.value, pubSignals[3]);
        checkAndInsertBid(pubSignals[3], msg.sender);
        emit BidSubmitted(msg.sender, pubSignals[3]);

    }



    function checkAndInsertBid(uint256 bid, address bidder) internal returns (bool) {
        for (uint256 i = 0; i < winners.length; i++) {
            if (bid > winingBids[i]) {
                for (uint256 j = winners.length - 1; j > i; j--) {
                    winners[j] = winners[j - 1];
                    winingBids[j] = winingBids[j - 1];
                }
                winingBids[i] = bid;
                winners[i] = bidder;
                return true;
            }
        }
        return false;
    }
}
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

    uint256 winingBid;
    address winner;
    address biddingToken;

    // what type of auction?
    // check the users collaretal
    bool private initialized = false;

    function initialize(
        address _verifier,
        uint256 _biddingDealine,
        uint256 _submissionDeadline,
        uint256 _ResultDeadline,
        uint256 _merkleRoot,
        uint256 _ceremonyId,
        uint256 _stateRoot,
        address _biddingToken
    ) external {
        verifier = Groth16Verifier(_verifier);
        merkleRoot = _merkleRoot;
        stateRoot = _stateRoot;
        biddingDeadline = _biddingDealine;
        bidSubmissionDeadline = _submissionDeadline;
        resultDealine = _ResultDeadline;
        ceremonId = _ceremonyId;
        biddingToken = _biddingToken;

        initialized = true;
    }

    function submitBid(
        uint256[2] calldata proofA,
        uint256[2][2] calldata proofB,
        uint256[2] calldata proofC,
        uint256[6] calldata pubSignals
    ) external returns (address) {
        if (block.timestamp < biddingDeadline) revert CastingPeriodEnded(biddingDeadline, block.timestamp);
        if (block.timestamp > bidSubmissionDeadline) {
            revert SubmissionPeriodEnded(bidSubmissionDeadline, block.timestamp);
        }
        uint256 bidderBalance = IERC20(biddingToken).balanceOf(msg.sender);
        // check the balanlce check the hash block of the stating vote

        if (bidderBalance != 1) revert();
        // check the hash of the block
        // chcec

        // check bidding token
        // how to manage the nonce of the user
        // they have to  create a fresh address
        // burning eth can be done for an auctuixp
        // that means no transaction for the user
        // user allocated amount
        // No way?
        // we need them to commit to the bid
        // complete based on the circuits signals
        // that requires the user address being checked
        // if (bid > winingBid) {
        //     winningBid = bid;
        //     winner = msg.sender;
        // }
        // i need to control the number of burn transactions this user does
        // which i praticly the same as controling his number of transactions
    }
}

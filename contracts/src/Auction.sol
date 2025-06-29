// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./verifier.sol";
import "./Errors.sol";

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
        uint256 _stateRoot
    ) external {
        verifier = Groth16Verifier(_verifier);
        merkleRoot = _merkleRoot;
        stateRoot = _stateRoot;
        biddingDeadline = _biddingDealine;
        bidSubmissionDeadline = _submissionDeadline;
        resultDealine = _ResultDeadline;
        ceremonId = _ceremonyId;

        initialized = true;
    }

    function submitBid(
        uint256[2] calldata proofA,
        uint256[2][2] calldata proofB,
        uint256[2] calldata proofC,
        uint256[6] calldata pubSignals
    ) external returns (address){
        if (block.timestamp < biddingDeadline) revert CastingPeriodEnded(biddingDeadline, block.timestamp);
        if (block.timestamp > bidSubmissionDeadline) revert SubmissionPeriodEnded(bidSubmissionDeadline, block.timestamp);
        // complete based on the circuits signals

        // if (bid > winingBid) {
        //     winningBid = bid;
        //     winner = msg.sender;
        // }
    }
}

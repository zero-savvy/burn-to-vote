// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./verifier.sol";
import "./Errors.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuardTransient.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

contract Auction is ReentrancyGuardTransient {
    Groth16Verifier public verifier;

    uint256 merkleRoot;
    uint256 stateRoot;
    uint256 ceremonyId;

    uint256 biddingDeadline;
    uint256 bidSubmissionDeadline;
    uint256 resultDealine;

    uint256[] winingBids;
    address[] winners;


    event BidSubmitted(address indexed bidder, uint256 bid);
    bool private initialized = false;
    mapping(uint256 => bool) public usedNullifiers;
    mapping(address => uint256) public collateral;


    function initialize(
        address _verifier,
        uint256 _biddingDealine,
        uint256 _submissionDeadline,
        uint256 _ResultDeadline,
        uint256 _merkleRoot,
        uint256 _ceremonyId,
        uint256 _stateRoot,
        uint256 _maxWinners
    ) external {
        verifier = Groth16Verifier(_verifier);
        merkleRoot = _merkleRoot;
        stateRoot = _stateRoot;
        biddingDeadline = _biddingDealine;
        bidSubmissionDeadline = _submissionDeadline;
        resultDealine = _ResultDeadline;
        ceremonyId = _ceremonyId;
        winners = new address[](_maxWinners);
        winingBids = new uint256[](_maxWinners);
        initialized = true;
    }

    function submitBid(
        uint256[2] calldata proofA,
        uint256[2][2] calldata proofB,
        uint256[2] calldata proofC,
        uint256[6] calldata pubSignals
    ) external payable{
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


    function results() external view returns (address[] memory, uint256[] memory) {
        if (block.timestamp < resultDealine) revert TallyNotAllowd();
        return (winners, winingBids);
    }


    function withdrawCollateral() external {
        if (block.timestamp < resultDealine) revert TallyNotAllowd();
        if (msg.sender == winners[0]) revert WinnerCannotWithdraw();
        payable(msg.sender).transfer(collateral[msg.sender]);
    }
}
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Voting {

    uint16 public totalVotes;
    function submitVote(uint8 _vote) public {
        require(_vote == 0 || _vote == 1, "Invalid vote");
        totalVotes = totalVotes + _vote;
        
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Erc20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract AuctionErc20 is Script {
    Erc20 public token;

    function setUp() public {}

    function run(
        string name,
        string symbol
    ) public {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(privateKey);

        uint256 b = vm.addr(privateKey).balance;
        console.log("balance", b);

        verifier = new Groth16Verifier();
        votingFactory = new VotingFactory();
        address voting = votingFactory.deployVotingContract(
            salt,
            VotingFactory.CeremonyType.Binary,
            address(verifier),
            submissionDeadline,
            tallyDeadline,
            merkleRoot,
            ceremonyId,
            stateRoot
        );

        vm.stopBroadcast();
        console.logAddress(address(voting));
    }
}

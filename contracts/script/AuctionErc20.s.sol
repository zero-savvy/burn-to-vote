// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {AuctionErc20} from "../src/AuctionErc20.sol";

contract TokenScript is Script {
    AuctionErc20 public auctionErc20;

    function setUp() public {}

    function run(address[] memory _whiteList) public {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(privateKey);

        uint256 b = vm.addr(privateKey).balance;
        console.log("balance", b);

        auctionErc20 = new AuctionErc20(_whiteList);

        vm.stopBroadcast();
        console.logAddress(address(auctionErc20));

        for (uint256 i = 0; i < _whiteList.length; i++) {
            uint256 tokenBalance = auctionErc20.balanceOf(_whiteList[i]);
            console.log("Token balance of", _whiteList[i], ":", tokenBalance);
        }
    }
}

// cd contracts && forge script TokenScript --rpc-url http://127.0.0.1:8545 --broadcast --sig 'run(address[])' [0x670F1836dfe9649c4C953721A4000a9858aBbDD9,0xe860c028D17501584c1f2A93f19E482132907D7A]

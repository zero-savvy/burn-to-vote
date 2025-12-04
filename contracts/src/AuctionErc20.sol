// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "./errors.sol";

contract AuctionErc20 is ERC20 {
    constructor(address[] memory _whitelist) ERC20("WhitelistToken", "WT") {
        for (uint256 i = 0; i < _whitelist.length; i++) {
            if (_whitelist[i] == address(0)) revert InvalidAddress();
            if (balanceOf(_whitelist[i]) != 0) revert DuplicateAddress(_whitelist[i]);
            _mint(_whitelist[i], 1 ether);
        }
    }
}

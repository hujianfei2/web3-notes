// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;
/**
* @title Counter
*/
contract Counter {
    uint256 public count;

    function get() public view returns (uint256) {
        return count;
    }

    function inc() public {
        count += 1;
    }

    function dec() public {
        // This function will fail if count = 0
        count -= 1;
    }
![image](https://github.com/user-attachments/assets/836c563f-57a7-47d0-ad93-328774227b24)

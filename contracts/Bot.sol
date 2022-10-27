// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity 0.8.17;

contract Bot {
    address owner;

    constructor() {
        owner = msg.sender;
    }

    modifier onlyOwner() {
        require(msg.sender == owner);
        _;
    }

    function frontrun_bytes(address _to, bytes memory data, uint _gas)
        public
        onlyOwner
        returns (bool, bytes memory)
    {
        uint balance_before = address(this).balance;

        (bool success, bytes memory output) = _to.call{gas: _gas}(data);
        require(success, "Tx failed");

        uint balance_after = address(this).balance;
        require(balance_before < balance_after, "Unprofitable frontrun");
        return (success, output);
    }
    
    receive() external payable{}
}

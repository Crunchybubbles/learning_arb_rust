pragma solidity ^0.8.7;

contract Testcontract {
  uint value;

  function setvalue(uint x) public {
    value = x;
  }

  function getvalue() view public returns (uint) {
    return value;
  }
}

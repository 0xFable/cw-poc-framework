# cw-test-harness 

A Rust package designed to provide an importable test harness into your tests :-) 

## Quick Start

The package contains simple things you can import to speed up your testing effort.

```rust

```

## Design 
The harness can be imported in your test scripts to reduce the amount of code you need to write for tests. Additionally a number of helpful common integration functions can be imported to again save on some code for simple actions such as setting up ContractWrappers on CW20s, providing or storing custom codes into the test harness environment or manipulating the environment such as advancing blocks to test the effect of time-senstive actions
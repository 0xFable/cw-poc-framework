# cw-poc-framework

Quickly create exploits or audit findings as-code which are both reproducible and quickly verifiable using a full suite of instantiated contracts.

## What this framework is for
This framework was made for security researchers, to facilitate a fast and convenient development of Proof-of-Concepts for bugs in Terra Cosmwasm smart contracts or even any CW contract. The framework heavily uses the brilliant `cw-multi-test` to provide an encapsulated suite of tools to simulate blockchain environments as  `Apps` and allows for exploits to be developed locally, in pure Rust, and then tested on Testnet or Mainet. 

#### Setup cargo package 
Create a new cargo package playground: 

`mkdir exploit-playground`

`cd exploit-playground`

`cargo init`

And then import both the framework and your local set of cargo contracts. Note the `path` param must be used when working with local cargo contracts:

```toml
[dependencies]
cw-poc-framework = {version="0.1.0"}
protocol-that-integrates-with-terraswap = { version = "0.0.0", path = "./location_to_my_cool_local_contract" }
```

The above will ensure that you have access to all the tools within the POC framework, the protocol you intend to work with and also a bunch of test related tools for cosmwasm. From here we can start to define some exploits-as-code!

#### Writing your first coded exploit 


##### Defining Scenarios 

Scenarios is the opinionated term used to create a repeatable environment with certain entities and rules established. They are crucial in proving out your discoveries in code if you do not want to expose the public to a potential vuln with your failed probing attempts. With this in mind, you can create a scenario which represents the exact environment you intend to perform your tests against. This helps provide you the auditor or exploiter with some assurance that when you bring your discovery on chain it will remain valid. 

Scenarios provide a very valuable way to look at testing too :-)

##### Writing custom controllers 

Controllers are provided by the dependency `cw-test-harness` which are used to help create the environment defined in your scenarios. The provided controllers can only get you so far and if you are interacting with many contracts for a single protocol you may find it advantageous to use the controller pattern for code reuse. Ideally use the provided traits where applicable to help guide you. 



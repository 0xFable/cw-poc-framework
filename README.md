# cw-poc-framework
A framework for creating PoC's for Cosmwasm Smart Contracts in a painless and intuitive way. Based on solana-poc-framework

Maybe this will be renamed to terra-poc-framework as right now its main focus is on cosmwasm based contracts in Terra and currently uses a Terra based fork of some libraries. 

**DISCLAIMER: any illegal usage of this framework is heavily discouraged. Most projects on Terra or chains which use CosmWasm offer a more than generous bug bounty.**

## Usage
To get started, just add the following line to the `dependencies` section in your `Cargo.toml`:
```toml
[dependencies]
cw-poc-framework = "0.1.0"
```
This crate already re-exports every Terra dependency you should need and most CW ones.

## What this framework is for
This framework was made for security researchers, to facilitate a fast and convenient development of Proof-of-Concepts for bugs in Terra Cosmwasm smart contracts or even any CW contract. The framework heavily uses the brilliant `cw-multi-test` to provide an encapsulated suite of tools to simulate blockchain environments as  `Apps` and allows for exploits to be developed locally, in pure Rust, and then tested on Testnet or Mainet. 

## Feature overview

### Mocks

A series of full-functionality contract mocks are saved for multiple protocols with the intent of making it relatively easily to replicate a given environemnt which most closely reflects the environment your test project has. For example, lets say you are auditing, pentesting or otherwise doing some discovery into exploits on a protocol that is heavily integrated with Terraswap; in order to do this in a local prototyping manner we would need to manually collect, store and deploy various contracts in order for the protocol we want to test against to actually work. 

The provided mocks take away alot of the complexity that comes from creating a lab from which to develop exploits against a protocol and gives you more time to focus on actually testing the protocols features. 

### Exploits-as-Code

The poc-framework attempts to promote and advocate the development of exploits as a piece of code, or even better yet.. a repeatable test which can be shared with a dev team as a part of the bug report. Why is this important ? Considering the saying 'code is law'; it becomes hard for a dev team to outright deny or a bounty admin from paying out on a disclosure when you are providing the exploit in a somewhat 'clean-room' environment. Additionally once the exploit is confirmed, the dev team can expand on the exploit disclosure script to make a testcase in response. 

### App

App, which comes from `cw-multi-test` provides us an entry point into a simulated blockchain environment which you can, to some degree configure yourself.
To borrow from MultiTest's Design doc: 

The main entry point to the system is called `App`, which represents a blockchain app.
It maintains an idea of block height and time, which you can update to simulate multiple
blocks. You can use `app.update_block(next_block)` to increment timestamp by 5s and height by 1
(simulating a new block) or you can write any other mutator to advance more.

It exposes an entry point `App.execute` that allows us to execute any `CosmosMsg`
and it wraps it as an atomic transaction. That is, only if `execute` returns success, will the state
be committed. It returns the data and a list of Events on successful execution or an `Err(String)`
on error. There are some helper methods tied to the `Executor` trait that create the `CosmosMsg` for
you to provide a less verbose API. `instantiate_contract`,`execute_contract`, and `send_tokens` are exposed
for your convenience in writing tests. Each execute one `CosmosMsg` atomically as if it was submitted by a user.
(You can also use `execute_multi` if you wish to run multiple message together that revert all state if any fail).

The other key entry point to `App` is the `Querier` interface that it implements. In particular, you
can use `App.wrap()` to get a `QuerierWrapper`, which provides all kinds of nice APIs to query the
blockchain, like `all_balances` and `query_wasm_smart`. Putting this together, you have one `Storage` wrapped
into an application, where you can execute contracts and bank, query them easily, and update the current
`BlockInfo`, in an API that is not very verbose or cumbersome. Under the hood it will process all messages
returned from contracts, move "bank" tokens and call into other contracts.

You can create an App for use in your testcode like:

```rust
fn mock_app() -> App {
    let env = mock_env();
    let api = Box::new(MockApi::default());
    let bank = BankKeeper::new();

    App::new(api, env.block, bank, Box::new(MockStorage::new()))
}
```

Inside App, it maintains the root `Storage`, and the `BlockInfo` for the current block.
It also contains a `Router` (discussed below), which can process any `CosmosMsg` variant
by passing it to the proper "Keeper".

Note: This properly handles submessages and reply blocks.

Note: While the API currently supports custom messages, we don't currently have a way to handle/process them.

Rather than just redefine all of MultiTests features here, we aim to highlight only `App`. As you start to build exploits you may find the MultiTest docs most useful.

### Examples 

There are two ways you could use the framework, either simply importing it into the cargo project you intend to pentest against or make a new cargo package and import both this framework and whatever protocols you intend to work with (assuming you have access to source code). Here is an example covering method 2: 

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


```rust
use cw_poc_framework::mocks::{mock_app, init_apps};

fn main(){
    let mut app = mock_app();
    let contracts = init_contracts(app);
}
```
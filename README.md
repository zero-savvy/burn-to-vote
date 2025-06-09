# Anonymous Voting using proof-of-burn

A Rust-based implementation of a fully on-chain, anonymous voting protocol using Proof-of-Burn and zk-SNARKs. Voters burn tokens to unspendable addresses, generate a zero-knowledge proof, and submit their vote on Ethereum-compatible chains without sacrificing privacy or verifiability.

## Features

- **Fully On-Chain**: No trusted third parties or off-chain tallying—everything happens in smart contracts.
- **Anonymous & Coercion-Resistant**: Each burn address is a hash commitment (binding and hiding). Voters can cast multiple unlinkable burns to override coerced votes.
- **Flexible Voting Schemes**: Supports majority voting, token-weighted, quadratic, ranked-choice, or open-content ballots via on-chain tally logic.
- **Lightweight ZKPs**: Uses Circom + Groth16 for succinct proofs; avoids heavy homomorphic encryption or MPC overhead.
- **Rust CLI**: Convenient command-line interface powered by `structopt` and `tokio`.

## Repository Layout

```
.
├── Cargo.toml
├── README.md
├── src
│   ├── main.rs       # CLI entry point
│   ├── circuits      # Circom circuit handlers
│   ├── commands      # vote, tally, demo, onchain_demo handlers
│   ├── utils         # configuration parsing, helper functions
│   └── db            # simple on-disk storage for ceremony state
└── contracts         # Solidity contracts (verifier + voting logic)
└── circuits          # circom circuits (vote + burn address + nullifier + mpt)
```

## Prerequisites

Before setting up the project, ensure the following tools are installed:

- **Node.js** (>=16.0.0) - [Install Node.js](https://nodejs.org/)
- **Rust** (for Cargo) - [Install Rust](https://www.rust-lang.org/tools/install)
- **Homebrew** (for macOS) - [Install Homebrew](https://brew.sh/)
- **Circom** - [Install Circom](https://docs.circom.io/getting-started/installation/)

## Setup

To set up the project, follow these steps to get the project up and running on your local machine.

1. **Clone the Repository**:
    ```sh
    git clone git@github.com:zero-savvy/burn-to-vote.git
    cd burn-to-vote
    ```
2. **Install Project Dependencies**:
    ```sh
    npm install
    npm run install-deps
    ```

    This will:
    - Install all Node.js dependencies including Circomlib, ganache-cli and snarkjs.
    - Run additional setup scripts to install Rapidsnark in the circuits folder.

3. **Start Ganache Locally**:
    ```sh
    ganache
    ```

    This will start a local blockchain instance for testing.

## Usage

To vote you need to generate an new ceremony or use an existing one.
If you already have the voting ceremony id you can skip this step.

### 1. Initiate Ceremony

Configure a new voting instance with default config:

network options: Ganache, Sepolia, Ethereum 

```sh
 cargo run -- initiate --network [NETWORK]
```

- Generates initial parameters
```sh
Config {
     network,
     ceremony_id,
     chain_id,
     votingDeadline,
     tallyDeadline,
     stateRoot,
     result,
     white_list,
     yesVotes,
     noVotes,
     finilized,
}
```

### 2. Vote

Burn-to-vote flow: compute a burn address, nullifier, burn eth and generate a proof:

```sh
cargo run -- vote --amount [AMOUNT] --vote [VOTE] --revote [REVOTE-FLAG] --private-key [PRIVATE-KEY] --ceremony-id [CEREMONY-ID]
```

- `AMOUNT`: The amout of ETH to burn.
- `VOTE`:  vote value (e.g., 0 or 1 for yes/no).
- `REVOTE`:  revote flag value (e.g., 0 1 for revoting).
- `PRIVATE-KEY`: private key (for ZK burn transaction).
- `CEREMONY-ID`: The unique ceremony identifier.

If no ceremony is providede the vote is applied to the latest generated ceremony.

To get the list of available ceremonies:
 
```sh
cargo run -- ceremonies  
```

### 2. Tally

Calculates and prints the result of ceremony if the tally time has passed:

```sh
cargo run -- tally --ceremony-id [CEREMONY-ID]
```

- `CEREMONY-ID`: The unique ceremony identifier.

if no ceremony id is providede the tally is applied on the latest generated ceremony.


### 4. Demo

Runs an in-memory ceremony without on-chain dependencies (for testing):
- compiles the vote circuit (~ 40 min)
- creates a burn address
- creates a nullifier
- burns a predetermined eth amount
- adds the data to merkle tree
- creates the vote circuit inputs, zkey, verification key
- creates the vote proof
- verifies the proof off-chain


```sh
cargo run -- demo [PRIVATE-KEY]
```


### 5. Onchain Demo

Demonstrates fully on-chain interactions :

- deploys the voting smart contract
- submits a pregenerated test vote proof
- returns the tally results

```sh
 cargo run -- onchain-demo [PRIVATE-KEY]
```


## Smart Contract Details

The smart contracts in `contracts/` implement a factory pattern for deploying voting instances:

### VotingFactory Contract

The factory contract (`contracts/src/VotingFactory.sol`) is responsible for deploying new voting instances with the following parameters:

- **Verifier Address**: Deployed Groth16 verifier (BN254)
- **Voting Deadline**: UNIX timestamp to lock voting
- **Tally Deadline**: After this, anyone can call `tallyVotes()`
- **Merkle Root**: Allow-list root for eligible voters
- **State Root**: Ethereum state root at voting end (for verifying burn balances)
- **Ceremony ID**: Unique voting identifier
- **Salt**: Unique salt for deterministic contract address generation

Key functions:
- `deployVotingContract(salt, verifier, merkleRoot, stateRoot, votingDeadline, tallyDeadline, ceremonyId)`: Deploys a new voting contract instance
- `getVotingContractAddress(salt)`: Returns the deterministic address for a voting contract given its salt

### Voting Contract

Each deployed voting contract instance (`contracts/src/Voting.sol`) implements the core voting logic:

Key functions:
- `submitVote(proofA, proofB, proofC, [nullifier, voteValue, revoteFlag, stateRoot, merkleProof, ceremonyId])`: Submits a vote with a zero-knowledge proof
- `submitRevote(...)`: Overwrites a previous vote if the nullifier matches
- `tallyVotes()`: Computes and publishes the final outcome after the tally deadline

The factory pattern allows for:
- Deterministic contract addresses based on salt
- Multiple concurrent voting instances
- Gas-efficient deployment of new voting contracts
- Easy tracking of all deployed voting instances

## Circom Circuits

Under `circuits/`:

- **`Vote.circom`**:  
  - Computes burn address: `address == H(secret ∥ ceremonyID ∥ vote ∥ blindingFactor)`.  
  - Computes nullifier: `address == H(secret ∥ ceremonyID ∥ blindingFactor)`.  
  - Verifies Merkle Patricia inclusion (`stateRoot`, `accountRLP`, `accountProof`).  
- **`rlp.circom`**: Supporting subcircuits.


## Experimental Results

- **Scalability**: Supports >1 million simulated voters with constant-time tallying.
- **Gas Costs**: Proof verification ~200k gas per vote; tallying is O(1).
- **Proof Generation**: ≤2s per witness on a modern CPU.

## Reference
If you have used this repo to develop a research work or product, please cite our paper:

1. [Burn Your Vote: Decentralized and Publicly Verifiable Anonymous Voting at Scale](https://eprint.iacr.org/2025/1022)
```
@misc{cryptoeprint:2025/1022,
  author       = {Stefan Dziembowski, Shahriar Ebrahimi, Haniyeh Habibi, Parisa Hassanizadeh and Pardis Toolabi},
  title        = {Burn Your Vote: Decentralized and Publicly Verifiable Anonymous Voting at Scale},
  howpublished = {Cryptology {ePrint} Archive, Paper 2025/1022},
  year         = {2025}
}

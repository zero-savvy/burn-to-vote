# Proof-of-burn Anonymous Voting

## Abstract

This project implements a decentralized voting protocol that combines token burning mechanisms with zero-knowledge proofs to ensure vote privacy and weight verification. 

## Setup

To set up the project, Follow these steps to get the project up and running on your local machine.


1. **Clone the Repository**:
    ```sh
    git clone https://github.com/toolabi/POB-Anonymous-Voting.git
    cd POB-Anonymous-Voting
    ```

2. **Download and run ganache(to run locally)**:
    ```sh
    npm i ganache-cli
    ganche
    ```

3. **Add the deployer private key to Makefile deploy command**

4. **Deploy contract**:
    ```sh
    make deploy
    ```

5. **run circuit commands**:
    ```sh
    make trusted_setup
    make vote_circuit
    make vote_zkey
    make vote_vkey
    ```

6. **Generate a burn address**:
    ```sh
    cargo run -- burn-address
    ```
7. **Burn some ETH**:
    ```sh
    cargo run -- burn
    ```
8. **Vote**:
    ```sh
    cargo run -- vote
    ```
9. **check your vote proof**:
    ```sh
    cargo run -- verify
    ```


## Features

- **Vote Privacy**: Ensures complete privacy in proof generation and submission using zero-knowledge proofs.
- **Public Verifiability**: Allows public verification of vote weights without revealing individual votes.
- **Coercion Resistance**: Prevents vote selling and coercion through unique burn addresses tied to personal data.
- **Double Voting Prevention**: Uses nullifiers to prevent double voting and ensure each vote is unique.

## Protocol Overview

1. **Token Burning**: Users burn tokens to create irreversible, publicly verifiable vote weights.
2. **Zero-Knowledge Proofs**: Prove correct weight attribution while preserving vote privacy.
3. **Nullifier Construction**: Prevents double voting and allows public verification.



## Voting Process

1. **Burn Transaction**: Users burn tokens during the voting period, generating a unique burn address.
2. **Proof Generation**: Generate zero-knowledge proofs to prove token burn and vote validity.
3. **Proof Submission**: Submit proofs and votes to the smart contract for verification.

## Security

- **Replay Attack Prevention**: Nullifiers prevent resubmission of votes.
- **Privacy**: zk-SNARKs ensure voter identities and token amounts remain hidden.
- **Verifiable Results**: All proofs and votes are verified and tallied on-chain.



## Conclusion

This protocol provides a secure, private, and verifiable decentralized voting system using token burning and zero-knowledge proofs. It addresses key challenges in decentralized voting, including vote privacy, public verifiability, and double voting prevention.

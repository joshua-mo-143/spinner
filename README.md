# spinner: a simple cryptocurrency tumbler
This repository explores how you can apply zk-SNARKs by creating a cryptocurrency tumbler as a proof of concept.

## What is a tumbler?
A tumbler is essentially a cryptocurrency service that can be used to send money to other people while obfuscating the original address. The idea is that you can prove the deposit happened, but not who made the deposit.

How does it work?
- User A will commit some funds and a secret word into the tumbler
- The tumbler will then store a hash of the secret/commitment
- User A tells user B the secret word so that they can then retrieve the funds
- User B will then use the secret with the tumbler and retrieve their funds

A small fee is generally taken each time the service is used.

While crypto tumblers are technically not illegal to use, they fall into a legal grey area and have occasionally been used for potential criminal activities as well as being viewed as a mechanism by which you can do some tax dodging. While it is *technically* possible to turn this repo into a fully fledged crypto tumbler, if you're serious about production you might as well just use Tornado Cash - this repository is primarily used for educational purposes.

## How do zk-SNARKs play into this?
The whole idea of a crypto tumbler heavily levies Zero Knowledge techniques (and merkle trees - a data structure heavily used in crypto). It's vital we understand this part as it makes all the other parts make sense.

zk-SNARKs (Zero-Knowledge Succinct Non-interactive ARguments of Knowledge) are essentially short proofs that "prove" a fact about something through a series of mathematical calculations. zk-SNARKs generally require quadratic constraints and as such, you are only able to express them in a limited number of ways. Generally, SNARKs are suitable for solving what are called `NP` problems - problems that can be easily proven, but not easily computed.

Check out the Circom circuits in the `circuit` folder if you're interested in the actual circuitry, but primarily speaking it mostly follows this logic:
- Get a leaf, a root, an element path and an indices path
- We then prove that these inputs lead to a given outcome
  - In this case, we compute what the new merkle root should be (by traversing the merkle tree and hashing each pair of leaves) and check if it is equal or not to the new merkle root
- If the constraints are all satisfied, return OK - if not, don't

You may notice if you check `src/tumbler.rs` that nothing actually happens besides the deposit when a user deposits. All of the ZK action happens withdrawal-side. Generally speaking, this is because there is nothing that needs to be proved when depositing funds. You could technically prove that a deposit happened, but there wouldn't really be a useful business reason to do so

zk-SNARKs

## Powers of Tau stuff
Get your ptau files [here](https://github.com/iden3/snarkjs?tab=readme-ov-file#7-prepare-phase-2) (might need to scroll down a bit).

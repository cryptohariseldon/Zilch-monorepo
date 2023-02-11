# Zilch-monorepo
Scripts and Examples to test Zilch ZK-rollups on Solana

Zilch is the first ever ZK-STARK rollup built on solana! Zilch utilises the MidenVM architecture to write provable programs, and thereafter verify them.
We chose Miden VM instead of other zk-STARK verifiers (such as winterfell), as Miden Assembly abstracts away the need to design computational-specific circuits. Moreover, Miden Assemly can serve as a high level compilation target for a host of languages, such as Rust, C++, Python. We leave building multi-language compilers for the MidenVM for future work.

## Zilch Architecture
At a high level, the Zilch ZK-rollup consists of five components. We are open sourcing all the components for maximum transparency, as we deploy Zilch Beta on Solana Devnet, in preparation for a mainnet launch.

1. Zilch - Proof generation: This utilises MidenVM to execute and generate a ZK-STARK proof for any arbitrary program written in Miden Assembly. Input: Miden Assembly Code, Inputs. Output: outputValues, ZK-STARK proof
2. Zilch - Proof writer client: We utilise a customised version of the solana-cli library, solana-cli-custom, to write proofs on-chain, to a buffer account. As proofs are typically 40-60KB, we utilise the inbuilt program writing functionality, to instead write to a data account- by breaking up the proof data into multiple transactions and write them at the appropriate offset.
Input: ZK-STARK proof, web3 provider; Output: Buffer Account Address where proof is stored.
3. Zilch - Verifier Program: A significantly updated version of Miden Verifier, this is especially built for fast and efficent, on-chain verification of Arbitrary ZK-proofs, on Solana. Input: Proof Account Address; Output: (Bool) isVerified.
4. Zilch - Verifier Client: A client to assist easy interaction with the on-chain Zilch Verifier Program.
5. Zilch Monorepo - examples and scripts: This repo, which provides easy scripts to access the above workflow, and contains examples for how Zilch can be used to prove different kinds of computations, on chain.

## Zilch Demo

Here we provide an end to end example for how to write, prove, and verify a computation using Zilch. We will utilise the fib8 script in this example. For more examples, please see the Examples folder.
<Coming Soon>

Step 1:

Generate Proof and write to file:
`cargo build`
`cargo run`

This will write serialised STARKPROOF to a local file called "proofd.bin". We will utilise this in the next step to write the proof on chain to a buffer account.

Links:
ZILCH - custom cli: https://github.com/cryptohariseldon/solana-custom
ZILCH - custom Miden Verifer: https://github.com/cryptohariseldon/miden-vm

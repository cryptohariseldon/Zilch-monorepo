# Zilch-monorepo
Scripts and Examples to test Zilch ZK-rollups on Solana

Zilch is the first ever ZK-STARK rollup built on solana! Zilch utilises the MidenVM architecture to write provable programs, and thereafter verify them.
We chose Miden VM instead of other zk-STARK verifiers (such as winterfell), as Miden Assembly abstracts away the need to design computational-specific circuits. Moreover, Miden Assemly can serve as a high level compilation target for a host of languages, such as Rust, C++, Python. We leave building multi-language compilers for the MidenVM for future work.

## Zilch Architecture
At a high level, the Zilch ZK-rollup consists of four components. We are open sourcing all the components for maximum transparency, as we deploy Zilch Beta on Solana Devnet, in preparation for a mainnet launch.

1. Zilch - Proof generation: This utilises MidenVM to execute and generate a ZK-STARK proof for any arbitrary program written in Miden Assembly. Input: Miden Assembly Code, Inputs. Output: outputValues, ZK-STARK proof
2. Zilch - Proof writer client: We utilise a customised version of the solana-cli library, solana-cli-custom, to write proofs on-chain, to a buffer account. As proofs are typically 40-60KB, we utilise the inbuilt program writing functionality, to instead write to a data account- by breaking up the proof data into multiple transactions and write them at the appropriate offset.
Input: ZK-STARK proof, web3 provider; Output: Buffer Account Address where proof is stored.
3. Zilch - Verifier Client: A client to assist easy interaction with the on-chain Zilch Orderbook Program.
4. Zilch Monorepo - examples and scripts: This repo, which provides easy scripts to access the above workflow, and contains examples for how Zilch can be used to prove different kinds of computations, on chain.

## Zilch Demo - basic

Here we provide an end to end example for how to write, prove, and verify a computation using Zilch. We will utilise the fib8 script in this example. For more examples, please see the Examples folder.
<Coming Soon>

In a different terminal window, start the local validator:
`solana-test-validator`

Step 1:

Generate Proof and write to file:
`cargo build`
`cargo run`

This will write serialised STARKPROOF to a local file called "proofd.bin". We will utilise this in the next step to write the proof on chain to a buffer account.

Step 2:
Write proof on-chain

Background:
Install custom cli
`git clone https://github.com/cryptohariseldon/solana-custom`
`cd solana-custom/cli`
`cargo build`
*note - the next commands will not work with the regular solana-cli, please install Zilch's custom solana-cli as explained above to proceed.

Write data generated in step-1 on-chain ,split over multiple txns.
`cargo run program write-buffer ../../proofd.bin`

If successful, You will get an output as follows:
`
checking check elf
bypassed check elf
Buffer: 4CN8ACDmTfchYRGYaiDDx6TVHLbQSk61jYNXuvaaN34x`

Great job, the proof is now written on-chain! Note the buffer account address, it will be utilised later in the verification steps.

## ZILCH Demo - miden cli + fib8 proof Verification

Clone the Zilch - custom miden repo:

Generate the proof file for fib8 Proof:

Write the proof on chain:
cargo run program write-buffer '/Users/dm/Documents/zilch_solana/custom-miden/miden-vm/miden/examples/fib/fib.proof'

Output :

checking check elf
bypassed check elf
Buffer: B6pwmZ5FrTkz2yWMWMkDcduzhojLggboDuNA7T4ReuvP

Read proof from on-chain account buffer, using client:

replace the proof account address in client/data.js:
`const accountAddress = 'B6pwmZ5FrTkz2yWMWMkDcduzhojLggboDuNA7T4ReuvP';` // replace with your local proof buffer account address shown above.

From the Zilch_monorepo folder, run client:
`node client/data.js`

Should display:
`Bytecode written to onchain_fib_trace.proof`

Next, use the custom miden library to verify the on-chain proof:
From the miden-custom folder -
`./target/release/miden verify --program-hash "c8653f31a1098e1b83c5d4972ec544cac00aa784bba18b5a9db7478977d38e68" --proof /Users/dm/Documents/zilch_solana/custom-miden/Zilch-monorepo/onchain_fib_trace.proof`





Links:
ZILCH - custom cli: https://github.com/cryptohariseldon/solana-custom
ZILCH - custom Miden Verifer: https://github.com/cryptohariseldon/miden-vm

#END-TO-END Examples

This shows an end to end example where:
- Alice wishes to get the fib8 computation executed with the public inputs 8, 16. She is willing to pay 10 Z-tokens for the completion of this computation.
- Bob is a compute provider that wishes to earn the tokens by submitting an on-chain ZK-STARK proof.

1. First, Alice prepares the program (.MASM), program hash, and public inputs. These are then deployed on chain. We provide client utilities for Alice to deploy her proof data.

input program: ./Alice/fib/fib.MASM
public inputs: ./Alice/fib/fib.inputs
input hash: "c8653f31a1098e1b83c5d4972ec544cac00aa784bba18b5a9db7478977d38e68"

a) compile program metadata
python3 -m 'from Scripts import inputs
inputs.Alice_publish_request("Alice/fib/fib.inputs", "Alice/fib/fib.masm", "c8653f31a1098e1b83c5d4972ec544cac00aa784bba18b5a9db7478977d38e68")'


b) Publish program metadata:
 `npx w3 put examples/Alice/compiled_inputs.json`

you will get a hash as output:
bafybeibzcnrrqc3ofg6yt5jdrr3rt5p2gczzi3otaq7bv4m6wmen6fhxhy

c) initialise escrow, and store metadata url on-chain:

escrow init

2.


Next, Bob, a compute provider, decides to accept Alices request. He copies the MASM code, an executes the code while generating a ZK-STARK trace as proof, off-chain, using the Miden VM library.

cargo run program write-buffer ../../proofd.bin



He then commits the outputs and proof produced on-chain. We provide a library, Solana-cli-custom to allow large proofs to be efficiently committed on-chain:


At this stage, any user, including Alice, can easily verify the on-chain proof as follows. The complexity of verification is exponentially lower than program execution complexity, and Alice can verify the proof even on a mobile phone for complex computations.


In case of any discrapency, a dispute may be raised by any user. If the proof is accepted, the reward of 10 tokens will be transferred to Bob.


This example scenario can be repeated with arbitrary code written in MASM. in the future, Masm will be a low level target for high level languages like rust and python - and devs will be able to write provable programs in languages most familiar to them!

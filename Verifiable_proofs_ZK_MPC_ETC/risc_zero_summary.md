# RISC ZERO

- It's now possible to pair a program's output with a self-certifying receipt, allowing a skeptical third party
  to verify correct execution — and the verifier doesn't need to repeat the original computation or even
  see the inputs to the program!

## Cool things with RISC-ZERO

- ZK means verifiable computation, this has huge impact on economy of computations that can be done off-chain by some
  party

Some use cases

- ZK Co-Processors - which enable blockchains to move the expensive part of their computation off-chain
- Optimistic Roll-Ups with ZK Fraud Proves(RISC ZERO works with Optimism team)

- RISC ZERO zkVM can prove execution of arbitrary code which allows devs to builds apps with Rust and C++
- zkVM is revolutionary because it allows building a ZK app without circuits and without a custom language.
- no background in advanced cryptography or mathematics is required

Things built with RISC ZERO:

- Zeth: prove the correct construction of an entire Ethereum block or an entire Optimism block
- Bonsai Pay: send Ethereum to someone's gmail address
- JSON: prove the contents of some entry in a JSON file, while keeping the rest of the data private
- Where's Waldo: prove that Waldo appears in a JPG file, while keeping the rest of the image private
- ZK Checkmate: prove that you see a mate-in-one, without revealing the winning move
- ZK Proof of Exploit: prove that you could exploit an Ethereum account, without revealing the exploit
- ECDSA signature verification: prove the validity of an ECDSA signature


- Not only dev is easy, performance is delivered - GPU accelration with CUDA and Metal (programs that enable using GPU
  acceleration for computation)
  and continuations for

Some more terms

- CUDA (Compute Unified Device Architecture) is a parallel computing platform and API developed by NVIDIA,
  enabling developers to run computations on NVIDIA GPUs.
  Why it’s used in ZKPs: CUDA allows ZK applications to perform proof computations in parallel across thousands of cores
  on NVIDIA GPUs,
  significantly speeding up the proof generation process for large-scale applications.
  Metal:

- Metal is Apple’s framework for GPU-accelerated applications on macOS and iOS, providing similar capabilities to CUDA
  but on Apple’s hardware.
  Why it’s used in ZKPs: Metal enables high-performance computations on Apple devices, allowing ZKP systems like RISC
  Zero’s zkVM to perform proof generation faster on Apple hardware.

- Continuations are a concept from computer science used to structure and manage sequences of tasks.

What it is in this context: In the RISC Zero zkVM, continuations allow for splitting a single large proof into smaller
sub-tasks that can be processed in parallel. This structure enables large computations to be "continued" or broken down
into smaller segments.
Why it’s valuable in ZKPs: Zero-knowledge proofs are often complex and computationally heavy. Continuations allow these
large programs to be processed in parallel segments, which is more efficient than working on them in a single,
monolithic task. This method enables quicker proof generation for larger programs by dividing the load among multiple
processing units (like GPUs) simultaneously.

# zkVM App

- uses existing Rust packages to prove correctness of execution

## Components

- Guest Code -> compiled to ELF binary
- Elf Binary
- Executor -> Runs ELF binary and records the session
- Session
- Prover -> Check and proves the validity of the session and outputs a receipt
- Receipt

- Anyone with the copy of the receipt can verify the correctness of the computation and can read publicly shared
  outputs. The verification algo
  receives an ImageID as a parameter, the ImageId serves as a Cryptographic Identifier for the expected ELF binary

## Deploying zkVM

- can be done on local machine or sending requests to Bonsai

## Receipts

- Result of the program along with proof that it ran correctly (succint validity prove of the correct execution of the
  program)
- Receipts can be passed to third-parties to validate the correctness

- consists of:
    - Journal: output of the program
    - Seal: opaque cryptographic part that attests to validity of the receipt

- Alice and Bob example:
    - both have access to guest program's source code (Bob needs to make sure Alice runs the code - so he extracts the
      image ID of the program)
    - inspect the receipt to extract the journal
    - verify the receipt to ensure that:
        - the execution was valid
        - the guest program that executed was consistent with the expected image ID

## Guest Code

- To build a zkVM application, we need our guest program to be able to:

    - read inputs,
    - write private outputs to the host, and
    - commit public outputs to the journal.
    - common lib methods here are listed

- Debugging
- Boilerplate code like ![no_std], ![no_main] and a macro for the entry function host needs to call

## Host Code

- the machines that running the zkVM
- it sets up the zkVM env and handles inputs and outputs to and from program
- if writing for Bonsai no need to write host code
- more details:
    - host will construct an env to have an executor run a guest program, here it will provide settings and communciate
      with the guest
    - run the prover to execute and prove the guest program and generate a receipt

example

```rust
    // IO is not shown here
use risc0_zkvm::{default_prover, ExecutorEnv}

let env = ExecutorEnv::builder().build().unwrap();
let prover = default_prover();
let receipt = prover.prove(env, METHOD_NAME_ELF).unwrap().receipt;
```

- Here, the zkVM uses METHOD_NAME_ELF binary to execute guest code. The METHOD_NAME_ELF is computed during compilation.
- The user needs to import it (use methods::{METHOD_NAME_ELF};) and then pass it as an input parameter to the
  prover.prove function.

- Verification method is also in the `risc0-zkvm Rust crate`.

```rust
    receipt.verify(METHOD_NAME_ID).unwrap();
```

# QA

Q: Why in hello_world example method folder is arranged as such?
A:

1. `cargo.toml` during build looks for `build.rs`, which contains a risc0 build method calls `embed_methods()` call.
2. `embed_methods()` looks at `cargo.toml` and the metadata called `methods` which a list of gues codes to load in to
   host"
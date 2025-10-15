Jun 12, 2025
Lecture 1
Interactive Proves

- Classical Proofs (Gauss, Euclid, Turing) --- Theorem, axioms -> drive proof
- Modern: prover/verifier, NP-proofs polynomical in length of claim (X is claim, |w| polynomical in X)
  Example:
- N is a product of 2 large primes
    - let's focus on time to verrify with the proof (later on proving time itself)
    - One way is prover sends p and q and Verifier sees if pq is prime and p give oroginal number and that p and q are
      prime but verifier learns about N, P and Q
    - in ZK, verifier asks different unpredictable questions, there is allowed to have a small probability of error
      threshold (so probabilistic) - verifier keeps posing questions to prover, with 1/2 probaility each time they get
      it
      right, until after many iterations it is probabilistically impossible to be right each time
- Not all Interactive Proofs can turn Non-Interactive
- different proving systems: interactive, simulated (Fiat-Shamir), more than 1 prover

Lecture 2 June 19, 2025
Non-Interactive Proofs

- SNARK: Succinct proof that certain statement is true. E.g. I know an m such that SHA256(m) = 0
- ZK-SNARK: same as definition above but never reveal m
- Companies:
    - Building SNARK Software: StarkWare, Aztec, MatterLabs, Espresso Systems
    - Using SNARK Software: RISC Zero, Scroll, Polygon, Aleo
    - SNARK Accelerator: SUPRA National, IGONYAMA
- A slow processor like L1 Blockchain can monitor herd of faster processors like GPU running unreliable SW (Babaei etal,
    1991)

    - do it in polylog time (very efficient)

- Examples (important to note that proofs are short and succinct and non-interactive)
    - Off-Chain computation
    - Bridging from one chain to another
    - Privacy Tx (Tornado Cash, ZKCash, Aleo, IronFish)
        - Proof that private Tx is compliant by banking laws (Espresso)
        - Proof that an exchange is solvent (Raposa)

    - Non-Blockchain
        - like fighting fake image in disinfo
        -
- Became possible because of polynomial/linear time proofs and algebra

- NARKs are non-interactive argument, SNARKS is Succinct versijn
- SNARKS formal def:
    - Given triple (S, P, V)
        - S gives Prover Params (PP) and Verifier Params (VP)
        - P operates on (PP, X , W) called proof (PI) where X is statement to prove and W is witness. Where strongly
          succinct
          mean len(pi) = O(log|C|)
        - V operates on (VP, X, PI) and needs to do so in O(log|C|)) where c is arithmetic circuit (gates as nodes and
          edges as operations to represent polynomials). V has no time to read C fully. This is where S giving VP comes
          in

- ZK-SNARK: is zero knowledge SNARK where w is not known.

- Circuits
    - Operate over Finite fields elements are mapped to others
    - Structured vs Non-Structured

- S - Pre-processor step
    - S(C;r) -> (PP, VP) , C = Circuit, r = random bits (r is kept hidden from prover otherwise prover can prove wrong
      statements)
    - often a trusted setup runs for the secret we want to generate proofs for and the machines that generated random
      secret r will be destroyed (often complicated and we would like to avoid)
    - another way is Universal (updatable) setup: secret r independet of C
      S init (lambda; r) -> gp (one-time setup, global params), S index (gp, C) -> (pp, vp)
    - Best: Transparent setup: S(C) does not use secret data (no trusted setup)

- SNARK Systems (Partial)
    - Groth'16 (trusted circuit), Plonk/Marlin (universal trusted setup)
    - For both proofs are extremely space and time efficient
    - Bulletproof: short proof, verification time bad, transparent system
    - STARK: relatively ok succint and verifier time, transparent setup
    - STARK is post-quantum
    - for all proof time is linear in size of circuit |C|
- Knowledge Soundness:
    - Prover knows w, if w can be "extracted" from P (prover)

- Build (ZK)SNARK
    - Two generic steps:
        - Functional Commitment - which is a cryptographic object whose security depends on certain assumptions
            - commitment : 2 Algorithms, it is like a sealed envole which is hiding and binding
                1. commit(m,r) -> com, where r is random bits
                2. verify(m, com, r) -> accept or reject

        - Functional Commitment types:
            - polynomial( KZG used most , because of constant time prover and verification), multi-linear,
              vector (e.g. Merkle), IPA (Inner Product Argument) -> important building blocks,
              you can build one from another
              -KZG10 a great impl example as its proof and verification is constant size independent of
              of degree d
            - Equality test protocol

        - Interactive Oracle Proof (IOP) - is an information theoric object

    - non-interactive
        - Fiat-Shemir Transform converts any public-coin interactive protocol to non-interactive

    - Combine with a layer of Interactive Oracle Polynomial
        - Terminology is such that if we apply IOP, functions are replaced by oracles (practically commitments) of those
          functions.
        - Polynomial-IOP Example

- The IOP Zoo - any of these combos gives SNARK
    - poly-IOP + poly commitment
    - multi-linear IOP + multi-linear commitment
    - Vector IOP + Merkle


- Coding SNARKs
    - developing circuits for programmers is untenable
    - Usually go from a DSL (like CAIRO, Zinc, Noir, Circom, etc.) to "SNARK friendly format" such as Circuit, R1CS,
      EVM ByteCode, etc. then this is applied to "SNARK Backend Prover" (Heavy Computation - given witness X
      produce Pi as proof)

Lecture 3 Aug 13

- Idea -> Highlevel Language -> Compiler/Library -> R1CS -> ZK Proof System
- examle: ZCash
  ZCash Circuit -> Bellman Lib -> R1CS -> Groth16

- Polynomial representation of circuit using DAG for example
- R1CS (Rank-1 Constraint Systems)   A x z (. -> element-wise multiplication) B x z = C x z, where x means inner product

- Circom -> Hardware Description Languages for R1CS - pros : gives control, but can be good or bad (most control, harder
  to work with)
- ArkWorks -> Rust highlevel language for R1CS (somewhere in between)
    - Bellman (Rust), Snarky (OCaml), PLONKISH (Rust), GadgetLib (C++)
- Zokrates -> better expressivity for R1CS (most elegant, cons: limited witness computation)
    - Noir, Leo, Cairo similar

- At the end of the day, all this languages provide these targets:
    - R1CS, Plonk, Air
    - They all use common techniques to represent: booleans, vars, structures, fix-width ints, mutation, control flow,
      etc.

- ZK Systems are going from idea to R1CS

Lecture 4

- SNARK Def -> Succinct Non-Interactive Argument of Knowledge, e.g. want to know m such that SHA256(m) = 0
- SNARK means proof is short and it is fast to verify
- IoP vs SNARK
    - Soundness (there is a witness that satisfies the claim) vs Knowledge Soundness (the Prover KNOWS the witness that
      satisfies the claim) (knowledge soundness is stronger and usually what we want - but not all the times both are
      meaningful)
    - IoP hard to do in certain apps like blockchain -> Fiat-Shamir turns IoP to non-interactive
- Merkle Tree -> examine relation to IoP and SNARK
  - 
- SZDL - Lamma (uni and multi variate polynomials) - multivariate prefered because uses many vars but keeps the degree
  low
-

## Terms

## 🧠 Key Terms with Descriptions and Examples

- Quadratic Residue
- bilinear pairing (used for proving)
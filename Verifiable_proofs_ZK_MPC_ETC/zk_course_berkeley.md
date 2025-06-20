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

- SNARKs

## Terms

# 📚 Foundational Terms in Zero-Knowledge Proofs (ZKP)

*Inspired by Prof. Shafi Goldwasser’s early lectures*

---

## 🧠 Key Terms with Descriptions and Examples

| **Term**                         | **Description**                                                                                                     | **Example**                                                                        |
|----------------------------------|---------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------|
| **Zero-Knowledge Proof (ZKP)**   | A method for one party (prover) to convince another (verifier) that a statement is true, **without revealing why**. | Prove a number is a quadratic residue mod N **without revealing its square root**. |
| **Prover (P)**                   | The party who knows the **witness** and tries to convince the verifier.                                             | Alice knows square root of a mod N.                                                |
| **Verifier (V)**                 | The party who checks the proof and is convinced that the statement is true.                                         | Bob wants to be sure a is a quadratic residue mod N.                               |
| **Statement / Instance**         | The public problem input, denoted as `x`.                                                                           | “Is `a` a square mod `N`?” or “Are two graphs isomorphic?”                         |
| **Witness**                      | The secret knowledge that shows the statement is true.                                                              | The square root `w` such that `w^2 ≡ a mod N`.                                     |
| **Language (L)**                 | The set of true statements; often an **NP language**.                                                               | All `a ∈ ℤ_N^*` that are quadratic residues.                                       |
| **NP**                           | Class of problems where verifying a proof takes polynomial time, but finding it may be hard.                        | Graph Isomorphism: easy to check if a mapping is valid.                            |
| **CO-NP**                        | A decision problem is in co-NP if "no" instances can be verified in polynomial time given a witness.                |                                                                                    |
| **Completeness**                 | If the statement is true, an honest prover **always** convinces the verifier.                                       | If Alice really knows the square root, Bob accepts.                                |
| **Soundness**                    | A dishonest prover **can’t cheat** and convince the verifier of a false statement.                                  | Alice cannot convince Bob that a non-residue is a residue.                         |
| **Zero-Knowledge**               | The verifier learns **nothing** except that the statement is true — not the witness.                                | Bob is convinced `a` is a residue, but learns no info about the root.              |
| **Simulator**                    | A tool used in security proofs to show that whatever the verifier sees could be faked.                              | Can simulate proof without the square root.                                        |
| **Extractor**                    | A theoretical algorithm that can extract the witness if the prover can cheat successfully.                          | If Alice can answer multiple challenges, we can recover the square root.           |
| **Interactive Proof**            | A multi-round protocol between prover and verifier.                                                                 | Graph Isomorphism protocol has 3 rounds.                                           |
| **Non-Interactive Proof**        | A proof that’s sent in one message, often using a hash-based challenge.                                             | zk-SNARKs use Fiat-Shamir to avoid interaction.                                    |
| **Commitment Scheme**            | Like a locked box: commit to a value, then open it later.                                                           | Alice commits to a value `r`, then reveals it after Bob's challenge.               |
| **Random Challenge**             | Chosen by verifier to prevent the prover from preparing ahead.                                                      | Bob sends a random bit in the graph isomorphism protocol.                          |
| **Rewinding**                    | A technique used in analysis to reset a protocol to an earlier point with a different challenge.                    | Used by extractor to get two responses from same commitment.                       |
| **Witness Indistinguishability** | Even if multiple witnesses exist, verifier can’t tell which one was used.                                           | If there are 2 square roots, verifier can't tell which one Alice used.             |

---

## 🧪 Example Protocols

### 1. Quadratic Residue (QR) Problem

- **Goal**: Prove that `a ∈ ℤ_N^*` is a quadratic residue without revealing the square root.
- **Protocol**:
    1. Prover chooses random `r` and sends `t = r² mod N`.
    2. Verifier sends a random bit `c ∈ {0,1}`.
    3. Prover responds with:
        - `s = r` if `c = 0`
        - `s = r * w mod N` if `c = 1`, where `w` is the square root of `a`.
    4. Verifier checks:
        - `s² ≡ t` if `c = 0`
        - `s² ≡ a·t` if `c = 1`

### 2. Graph Isomorphism Protocol

- **Goal**: Prover knows an isomorphism `π` between graphs `G₁` and `G₂` and wants to prove they are isomorphic without
  revealing `π`.
- **Protocol**:
    1. Prover picks random permutation `ρ` and sends permuted graph `H = ρ(G₁)`.
    2. Verifier sends a random challenge bit `c ∈ {1, 2}`.
    3. Prover reveals:
        - If `c = 1`: show `ρ`
        - If `c = 2`: show `ρ ◦ π⁻¹` to map `G₂ → H`
    4. Verifier checks that the mapping is a valid isomorphism to `H`.

---

*These definitions and examples build the foundation for understanding more advanced zero-knowledge protocols like
zk-SNARKs, zk-STARKs, and ZK rollups.*

- 🔑 Key Differences: NP vs NP-Complete

| Concept                  | NP                                     | NP-Complete                                    |
|--------------------------|----------------------------------------|------------------------------------------------|
| What is it?              | Problems with **verifiable** solutions | The **hardest** problems in NP                 |
| Easy to solve?           | Maybe — depends on the problem         | Probably **not** (no one has found a fast way) |
| Easy to check?           | ✅ Yes                                  | ✅ Yes                                          |
| All of NP reduces to it? | ❌ Not necessarily                      | ✅ Yes                                          |

- ZK and Blockchain
  In theory: Prover and verifier exchange messages multiple times (interactive proofs).
  In practice (blockchain): We use non-interactive ZK proofs — especially: zk-SNARKs, zk-STARKs
  This is achieved using the Fiat-Shamir heuristic, which removes interaction by replacing verifier's randomness with a
  cryptographic hash function.

## Terms 
# 🧭 ZK + Rust Learning Roadmap (Functional Naming Edition)

---

## 🟦 Phase 1: ZK Math & Foundational Rust (Weeks 1–4)

| Week   | What You'll Learn                       | Project                                | Output                                      |
|--------|-----------------------------------------|----------------------------------------|---------------------------------------------|
| **W1** | Proof Conversations (Prover ↔ Verifier) | Interactive proof with secrets in Rust | Blog: “Proving Knowledge Without Revealing” |
| **W2** | Building Constraint Systems by Hand     | Encode algebra as constraints (R1CS)   | Blog: “Making a Circuit Speak Algebra”      |
| **W3** | Hiding & Binding Commitments            | Secret commit & reveal demo            | Code + write-up                             |
| **W4** | Designing Computations into Circuits    | Arithmetic circuit + witness generator | Blog: “How Circuits Simulate Code”          |

Readings:

- Berkerly BootCamp
-

---

## 🟨 Phase 2: Real ZK Libraries + Rust Power (Weeks 5–8)

| Week   | What You'll Learn                     | Project                                | Output                                   |
|--------|---------------------------------------|----------------------------------------|------------------------------------------|
| **W5** | Plug & Play Proof Systems             | Use Groth16 prover/verifier APIs       | Blog: “Plugging Rust into Proofs”        |
| **W6** | Running Programs with ZK Transparency | Split zkVM: write → run → prove        | Blog: “Making Programs Prove Themselves” |
| **W7** | Proving Correct Input/Output          | Full proof pipeline for data integrity | CLI + blog                               |
| **W8** | Bridging Visual Circuits to Rust Code | Verify Circom circuit from Rust        | Repo + walkthrough                       |

---

## 🟥 Phase 3: Advanced Rust, TEE, and Web3 Integration (Weeks 9–12)

| Week    | What You'll Learn                         | Project                              | Output                             |
|---------|-------------------------------------------|--------------------------------------|------------------------------------|
| **W9**  | Enforcing Code Trust with Hardware        | TEE-based randomness + proof         | Blog: “Hardware Enclaves for ZK”   |
| **W10** | Going Low-Level with Memory & Concurrency | Async-safe ZK service (Arc/Mutex)    | Repo + blog                        |
| **W11** | Automating Code Patterns with Macros      | `zk_prove!` procedural macro         | Blog: “Macros to Teach ZK in Rust” |
| **W12** | Making Proofs Usable in Web3              | Badge dApp that verifies user proofs | Capstone: blog + screencast        |

---

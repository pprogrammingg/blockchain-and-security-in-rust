# Goal

To do an end-to-end hands-on project in the Zero-Knowledge, payment and e-mail domain using Rust for the most part.
Output of work is hostable and can be used in real-time.

# Breakdown

| Stage                                                        | Complexity     | Est. time (solo) | ZK / non-ZK       | Notes                                                  | Task Status |
|--------------------------------------------------------------|----------------|------------------|-------------------|--------------------------------------------------------|-------------|
| Learn DKIM parsing and how to extract public keys from DNS   | 🟢 Easy        | 1–2 days         | Non-ZK            | Many Rust crates exist                                 | COMPLETED   |
| Write Rust code to verify DKIM signature                     | 🟡 Medium      | 3–5 days         | Non-ZK            | Can base on open-source DKIM libraries                 |             |
| Integrate DKIM verification into a RISC Zero guest program   | 🔵 Harder      | 1–2 weeks        | ZK                | Need to fit verification logic inside zkVM constraints |             |
| Deploy verifier contract + integrate blockchain call         | 🟡 Medium      | 3–5 days         | ZK                | RISC Zero provides verifier template                   |             |
| Build backend that coordinates email → proof → on-chain call | 🟢 Easy–Medium | 1 week           | ZK                | Straightforward Rust web service                       |             |
| Polish prototype (UI, error handling, etc.)                  | 🟢 Easy        | 3–5 days         | Non-ZK / optional | Optional                                               |             |


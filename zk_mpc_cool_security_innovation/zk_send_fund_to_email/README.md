# Goal

To do an end-to-end hands-on project in the Zero-Knowledge, payment and e-mail domain using Rust for the most part.
Output of work is hostable and can be used in real-time.

# Breakdown of Tasks

| Stage                                                        | Complexity     | Est. time (solo) | ZK / non-ZK       | Notes                                                  | Task Status |
|--------------------------------------------------------------|----------------|------------------|-------------------|--------------------------------------------------------|-------------|
| Learn DKIM parsing and how to extract public keys from DNS   | 🟢 Easy        | 1–2 days         | Non-ZK            | Many Rust crates exist                                 | COMPLETED   |
| Write Rust code to verify DKIM signature                     | 🟡 Medium      | 3–5 days         | Non-ZK            | Can base on open-source DKIM libraries                 |             |
| Integrate DKIM verification into a RISC Zero guest program   | 🔵 Harder      | 1–2 weeks        | ZK                | Need to fit verification logic inside zkVM constraints |             |
| Deploy verifier contract + integrate blockchain call         | 🟡 Medium      | 3–5 days         | ZK                | RISC Zero provides verifier template                   |             |
| Build backend that coordinates email → proof → on-chain call | 🟢 Easy–Medium | 1 week           | ZK                | Straightforward Rust web service                       |             |
| Polish prototype (UI, error handling, etc.)                  | 🟢 Easy        | 3–5 days         | Non-ZK / optional | Optional                                               |             |

# Email → ZK Login / Crypto Withdrawal Flow

1. **User enters email** in the frontend UI.

2. **Backend generates a nonce** and sends an **email** containing the nonce to the user inbox.

3. **Frontend / prover.wasm collects inputs**:
    - `raw_email` (headers + body - backend already canonicalize it so ZK prover does not need to do that)
        - Backend (or prover side JS/WASM) does:
            - unfold headers
            - canonicalize headers (relaxed/simple)
            - canonicalize body
            - extract DKIM params
            - produce exactly the byte sequences to hash

    - `user_email` (from UI)
    - `user_nonce` (entered/copied from email)
    - `expected_nonce` (provided by backend as Poseidon(user_nonce))
    - `zkEmailId` = Poseidon(user_email) (provided by backend or computed locally)

4. **prover.wasm executes ZK circuit**:
    - **DKIM verification**: confirms email came from claimed domain
    - **Nonce verification**: ensures `user_nonce` matches `expected_nonce`
    - **zkEmailId computation**: confirms Poseidon(user_email) = zkEmailId

5. **prover.wasm outputs**:
    - `zkProof` (proof of email ownership)
    - `zkEmailId` (public identifier)

6. **Smart contract (e.g. Radix component) verifies proof**:
    - Marks `isVerified[zkEmailId] = true` **or** mints NFT **or** credits funds to `zkEmailId` balance

7. **User now has access / can withdraw crypto**:
    - Funds mapped to zkEmailId
    - No email, nonce, or raw message is revealed on-chain

# Generating and Signing with Private Key (levels of ease and security)

# Transaction Signing: User Friction vs Security

| Scenario                                          | Ease for User (Least → Most Friction) | Security (Most → Least) | Notes                                                                                                                                                                                         |
|---------------------------------------------------|---------------------------------------|-------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Web-only (browser)                                | 1 (easiest)                           | 3 (lowest)              | No extra apps/extensions required. Key is generated in the browser and stored in IndexedDB/localStorage. Very easy UX, but vulnerable if browser is compromised or device is stolen.          |
| Mobile browser with native bridge (WebView + app) | 2                                     | 2                       | Requires native app installation to store keys in Keystore/Secure Enclave. Signing via JS bridge is smooth once app is installed. Security is strong because the key never leaves the device. |
| Hardware wallet / browser extension               | 3 (highest friction)                  | 1 (highest)             | User must have wallet or extension, sometimes extra steps (connect, confirm). Security is highest; private key never leaves hardware. Signing is isolated from the browser.                   |

# 🔐 Cryptography Landscape & Use-Case Map

## 🧱 1. Core Cryptographic Primitives

| Category                 | Examples               | Purpose                                       |
|--------------------------|------------------------|-----------------------------------------------|
| 🔑 Symmetric Encryption  | AES, ChaCha20          | Fast encryption/decryption with shared secret |
| 🔓 Public-Key Encryption | RSA, ECC, Kyber        | Secure key exchange, message encryption       |
| 🧾 Digital Signatures    | ECDSA, Ed25519         | Authentication, non-repudiation               |
| 🧮 Hash Functions        | SHA-256, BLAKE3        | Fingerprinting, data integrity                |
| 🔐 MACs                  | HMAC, Poly1305         | Message integrity + authentication            |
| 🔁 KDFs                  | PBKDF2, scrypt, Argon2 | Strengthen weak secrets (e.g. passwords)      |

---

## 🧰 2. Advanced Cryptographic Protocols

| Technique                        | Examples                    | Used for                           |
|----------------------------------|-----------------------------|------------------------------------|
| 🔍 Zero-Knowledge Proofs         | zk-SNARKs, STARKs           | Privacy-preserving proofs          |
| 👥 MPC (Multi-Party Computation) | SPDZ, GMW                   | Collaborative private computation  |
| 🧮 Homomorphic Encryption        | BFV, CKKS, TFHE             | Compute on encrypted data          |
| 🔒 TEEs                          | Intel SGX, ARM TrustZone    | Hardware-isolated secure computing |
| 🧙 Secret Sharing                | Shamir’s Secret Sharing     | Splitting secrets among parties    |
| 👻 Steganography                 | Hiding text in images/audio | Concealing message existence       |

---

## 🌐 3. Real-World Systems

### 🧱 Blockchain (e.g. Ethereum, Zcash, Mina)

| Component                   | Crypto Used                         |
|-----------------------------|-------------------------------------|
| Wallets                     | Asymmetric keys (ECDSA, EdDSA)      |
| Transactions                | Digital signatures, hashing         |
| Privacy (Zcash, Mina)       | zk-SNARKs, zk-STARKs                |
| Consensus                   | Hashes (PoW), signatures (PoS, BLS) |
| Smart Contract Verification | ZK proofs, Merkle trees             |

---

# 🔐 Cryptography Landscape & Use-Case Map

## 🧱 1. Core Cryptographic Primitives

| Category                 | Examples               | Purpose                                       |
|--------------------------|------------------------|-----------------------------------------------|
| 🔑 Symmetric Encryption  | AES, ChaCha20          | Fast encryption/decryption with shared secret |
| 🔓 Public-Key Encryption | RSA, ECC, Kyber        | Secure key exchange, message encryption       |
| 🧾 Digital Signatures    | ECDSA, Ed25519         | Authentication, non-repudiation               |
| 🧮 Hash Functions        | SHA-256, BLAKE3        | Fingerprinting, data integrity                |
| 🔐 MACs                  | HMAC, Poly1305         | Message integrity + authentication            |
| 🔁 KDFs                  | PBKDF2, scrypt, Argon2 | Strengthen weak secrets (e.g. passwords)      |

---

## 🧰 2. Advanced Cryptographic Protocols

| Technique                        | Examples                    | Used for                           |
|----------------------------------|-----------------------------|------------------------------------|
| 🔍 Zero-Knowledge Proofs         | zk-SNARKs, STARKs           | Privacy-preserving proofs          |
| 👥 MPC (Multi-Party Computation) | SPDZ, GMW                   | Collaborative private computation  |
| 🧮 Homomorphic Encryption        | BFV, CKKS, TFHE             | Compute on encrypted data          |
| 🔒 TEEs                          | Intel SGX, ARM TrustZone    | Hardware-isolated secure computing |
| 🧙 Secret Sharing                | Shamir’s Secret Sharing     | Splitting secrets among parties    |
| 👻 Steganography                 | Hiding text in images/audio | Concealing message existence       |

---

## 🌐 3. Real-World Systems

### 🧱 Blockchain (e.g. Ethereum, Zcash, Mina)

| Component                   | Crypto Used                         |
|-----------------------------|-------------------------------------|
| Wallets                     | Asymmetric keys (ECDSA, EdDSA)      |
| Transactions                | Digital signatures, hashing         |
| Privacy (Zcash, Mina)       | zk-SNARKs, zk-STARKs                |
| Consensus                   | Hashes (PoW), signatures (PoS, BLS) |
| Smart Contract Verification | ZK proofs, Merkle trees             |

---

### 🛡 VPNs (e.g. OpenVPN, WireGuard)

| Component            | Crypto Used                           |
|----------------------|---------------------------------------|
| Session Key Exchange | Public key (DH, Curve25519)           |
| Traffic Encryption   | Symmetric encryption (AES, ChaCha20)  |
| Authentication       | Certificates (X.509, RSA, ECC)        |
| Integrity            | HMAC, AEAD (authenticated encryption) |

---

### 📩 Secure Messaging (e.g. Signal, WhatsApp)

| Component             | Crypto Used                     |
|-----------------------|---------------------------------|
| Identity Verification | Asymmetric keys (Ed25519)       |
| Message Encryption    | Symmetric (AES, Double Ratchet) |
| Forward Secrecy       | Ephemeral keys (X3DH, DH)       |
| Authentication        | MACs, Digital signatures        |
| Metadata protection   | Sealed sender, ZK if advanced   |

---

## 📊 4. How They Interact (Workflow Diagram)

[Password] → [KDF] → [Symmetric Key] → [Encrypt File]
↓
[MAC] ← [Message Integrity]

[User A Public Key] ←→ [User B Public Key]
↕ ↕
[Sign Message] [Verify Signature]

[Data] → [ZK-Proof] → [Blockchain] (proves without revealing data)


---

## 🎯 TL;DR Summary

- **Public key crypto** enables identity, secure key exchange, and digital signatures.
- **Symmetric crypto** is fast and efficient for bulk data once keys are shared.
- **Hashing, MACs, and KDFs** ensure integrity, authentication, and safe key handling.
- **ZKPs, MPC, and FHE** are powerful privacy-preserving computation tools for Web3 and beyond.

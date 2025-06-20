# Cryptographic Algorithms in Blockchain

| Algorithm   | Type                                       | Curve Used / Function          | Signature Size                        | Speed                    | Security Features                                        | Use Cases                                                       |
|-------------|--------------------------------------------|--------------------------------|---------------------------------------|--------------------------|----------------------------------------------------------|-----------------------------------------------------------------|
| **ECDSA**   | Elliptic Curve Digital Signature Algorithm | secp256k1 (Bitcoin, Ethereum)  | 64 bytes                              | Slower than EdDSA        | Vulnerable to poor randomness & malleability issues      | Used in **Bitcoin, Ethereum**                                   |
| **EdDSA**   | Edwards-Curve Digital Signature Algorithm  | Ed25519, Ed448                 | 64 bytes (Ed25519), 114 bytes (Ed448) | Faster than ECDSA        | Deterministic, resistant to side-channel attacks         | Used in **Solana, Zcash, Monero**                               |
| **Schnorr** | Elliptic Curve Signature Scheme            | secp256k1 (Taproot in Bitcoin) | 64 bytes                              | Faster verification      | Supports **multi-signatures**, better privacy than ECDSA | Used in **Bitcoin (Taproot), Blockchains favoring aggregation** |
| **BLS**     | Boneh-Lynn-Shacham Signature Scheme        | BLS12-381                      | 96 bytes                              | Slower than Schnorr      | Supports **signature aggregation**, non-malleable        | Used in **Ethereum 2.0, Dfinity, ZK blockchains**               |
| **Keccak**  | Cryptographic Hash Function                | Keccak-256                     | 32 bytes (256-bit) hash               | Fast, hardware optimized | Resistant to length-extension attacks, SHA-3 winner      | Used in **Ethereum hashing, digital signatures, Merkle trees**  |
| **RSA**     | Rivest-Shamir-Adleman                      | N/A                            | Large (256 bytes for 2048-bit key)    | Slow                     | Quantum-vulnerable, widely understood legacy system      | Rare in blockchains, but used in **older systems**              |

## When to Use What?

- **ECDSA**: Used in traditional blockchain networks like Bitcoin and Ethereum.
- **EdDSA**: Faster and more secure than ECDSA, used in privacy-focused and performance-oriented blockchains (Solana,
  Monero).
- **Schnorr**: Enhances privacy and efficiency, especially for multi-signature transactions (Bitcoin Taproot).
- **BLS**: Used in blockchains that require signature aggregation and threshold cryptography (Ethereum 2.0).
- **Keccak**: The primary hash function used in **Ethereum**, securing transactions, addresses, and Merkle trees.
- **RSA**: Not commonly used in modern blockchains due to large key sizes and inefficiency, but still used in legacy
  cryptographic systems.

# Types of Security Attacks in Blockchain

## **1. Network-Level Attacks**

| Attack             | Description                                                                                                                                                     | Impact                                                                       |
|--------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------|
| **51% Attack**     | A single entity gains control of more than 50% of the network’s mining/hash power, allowing them to rewrite transactions, double-spend, and block transactions. | Affects **PoW blockchains** (e.g., Bitcoin, Ethereum Classic).               |
| **Sybil Attack**   | An attacker creates multiple fake nodes to gain control over a decentralized network.                                                                           | Used to manipulate **P2P networks, voting, and consensus mechanisms**.       |
| **Eclipse Attack** | An attacker isolates a target node by surrounding it with malicious peers, preventing it from seeing the honest network.                                        | Can delay transactions, enable **double-spending** or front-running attacks. |
| **Routing Attack** | Exploits weaknesses in internet infrastructure (ISPs) to intercept blockchain traffic and split the network.                                                    | Can cause **chain forks**, **delayed transactions**, and weaken security.    |

---

## **2. Consensus-Level Attacks**

| Attack                | Description                                                                                                 | Impact                                                                   |
|-----------------------|-------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------|
| **Selfish Mining**    | Miners withhold valid blocks to gain an advantage by mining in secret and publishing blocks strategically.  | Reduces network security and **centralizes mining power**.               |
| **Nothing-at-Stake**  | Unique to **PoS blockchains**, validators can validate multiple forks, increasing the risk of chain splits. | Weakens **chain finality** and allows attackers to manipulate consensus. |
| **Long-Range Attack** | Attackers with historical stake can rewrite the blockchain by forking from a much earlier point.            | Affects **PoS blockchains**, breaking **immutability**.                  |

---

## **3. Smart Contract & DApp Attacks**

| Attack                         | Description                                                                                               | Impact                                                                        |
|--------------------------------|-----------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------|
| **Reentrancy Attack**          | A malicious contract repeatedly calls back into a vulnerable contract before state changes are finalized. | Used in the **2016 DAO hack** to drain **$60M** from Ethereum.                |
| **Integer Overflow/Underflow** | A math bug where integer values exceed their limit, causing unexpected behavior.                          | Can be used to **steal tokens** or manipulate **contract logic**.             |
| **Front-Running**              | Attackers exploit **pending transactions** by placing their own transactions with higher gas fees.        | Used in **DeFi, MEV (Miner Extractable Value)** to manipulate **DEX trades**. |

---

## **4. Cryptographic & Private Key Attacks**

| Attack                       | Description                                                                | Impact                                                  |
|------------------------------|----------------------------------------------------------------------------|---------------------------------------------------------|
| **Private Key Theft**        | Stolen or leaked private keys allow full access to blockchain assets.      | Leads to **irreversible asset loss**.                   |
| **Weak Key Attack**          | Exploits weak/random number generation in wallet key creation.             | Found in **Ethereum wallets**, leading to stolen funds. |
| **Man-in-the-Middle (MITM)** | Attackers intercept blockchain communication to alter transaction details. | Used in phishing attacks and **malicious wallet apps**. |

---

## **5. Economic & Governance Attacks**

| Attack                | Description                                                                                                                  | Impact                                                                     |
|-----------------------|------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------|
| **Flash Loan Attack** | Attackers take out large, uncollateralized loans and manipulate DeFi protocols before repaying them in a single transaction. | Used in **DeFi exploits**, leading to **millions in losses**.              |
| **Governance Attack** | Attackers buy enough governance tokens to manipulate protocol rules or drain treasury funds.                                 | Affects **DAOs and DeFi protocols** (e.g., Beanstalk hack - $182M stolen). |
| **Pump and Dump**     | Market manipulation where prices are artificially inflated before large sell-offs.                                           | Common in **low-liquidity tokens and ICO scams**.                          |

---

## **How to Defend Against These Attacks**

- **Use robust consensus mechanisms** (e.g., PoS with slashing, hybrid PoW/PoS).
- **Improve smart contract security** by using **formal verification**, audits, and best coding practices.
- **Implement multi-signature wallets** to prevent single-point failures in private key security.
- **Use secure randomness generation** for cryptographic operations.
- **Monitor for suspicious network behavior** (e.g., unusual mining activity, Sybil attacks).
- **Adopt Layer-2 scaling solutions** (e.g., rollups, state channels) to reduce MEV and front-running risks.


# OAuth 2.0 vs Open Id Connect (ODIC)
- OAuth provides access token to access resources (Authorization)
- ODIC in addition confirms the identity of the user (Authentication) and generates an Id Token on
top of granting access token to resources. So ODIC does Authentication and Authorization.
- ODIC is less prone to phishing attacks because of more rigour such as 2MFA, Biometrics, etc. used by
identity provider such as Google 

- Question: Is identity provided by a central entity a good thing? Can this be decentralized? Maybe UCAN is the answer? 

# UCAN and DIDs
- UCANs are authorization and can use DIDs as authentication
- UCAN can use cryptographic proofs for capabilities 
- UCAN is attunable 
- UCAN can let you compose and chain capabilities with zero coordination
- UCAN everything is in the token - does not need lookup databases, etc
- UCAN has abilities to revoke capability from Issuer to anyone down the chain





# **Categorization of Keys & Tokens in Web2 vs. Web3**

## **Web2 Authentication & Authorization**
| Type                  | Use Case | Security | Strengths | Weaknesses |
|----------------------|---------|----------|-----------|------------|
| **Session Tokens** (Cookies) | Web logins, stateful auth | Stored in browser (often with `httpOnly` flag) | Convenient for user sessions, auto-expiry | Susceptible to CSRF/XSS if not secured |
| **Bearer Tokens** (OAuth, JWT, OIDC)** | API authentication, SSO (e.g., Google login, OpenID Connect) | Signed & time-limited (JWT), but vulnerable if stolen | Works across domains, no session required | Can be replayed if intercepted, requires refresh mechanisms |
| **OIDC (OpenID Connect)** | Identity verification for logins (e.g., Google Sign-In, Okta) | Extends OAuth with ID tokens, cryptographic signatures | Eliminates password-based logins, reduces phishing | Still relies on identity providers (Google, Microsoft, etc.) |
| **API Keys** | Service-to-service auth (e.g., weather APIs) | Simple but weak security if leaked | Easy to implement, works without user login | Hard to revoke, often misused, visible in URLs |
| **HMAC-Signed Tokens** | Secure API requests (e.g., AWS Signature v4) | Signature-based verification prevents tampering | More secure than raw API keys | Requires signing complexity on client-side |
| **mTLS Certificates** | Strong authentication for enterprise APIs | Uses mutual TLS (client & server both verify) | Highly secure | Hard to manage, certificate expiration issues |


---

## **Web3 Authentication & Authorization**
| Type                  | Use Case | Security | Strengths | Weaknesses |
|----------------------|---------|----------|-----------|------------|
| **Private Keys (ECDSA, Ed25519)** | Signing transactions (Ethereum, Solana, etc.) | Extremely secure if stored properly (hardware wallet) | No third-party trust needed | If lost, funds are gone forever |
| **Seed Phrases (12/24 words)** | Wallet recovery (MetaMask, Ledger, etc.) | High security but must be kept secret | Enables full wallet recovery | Irrecoverable if lost, phishing risk |
| **Smart Contract Wallet Keys** | Smart contract-based auth (e.g., Safe Wallet) | Stored on-chain with recovery mechanisms | Programmable access control | Gas fees for key management |
| **DID (Decentralized Identity) Keys** | Self-sovereign identity (e.g., ENS, Ceramic) prove who you are, can be used as building blocks for UCANs| Uses cryptographic signatures (no central authority) | Portable across dApps | Adoption is still evolving |
| **Session Keys (Temporary Private Keys)** | Temporary key pairs for better UX (e.g., WalletConnect) | Used only for a session, reducing risk | Reduces need for constant wallet confirmations | If leaked, attacker can act for session duration |
| **UCAN (User-Controlled Authorization Networks)** | Delegated, decentralized authorization (IPFS, Web3 storage), declare what you can do, can utilize DIDs as authentication| Cryptographic capability-based system | No need for centralized identity provider | Adoption is still growing, complex UX |
| **Biscuit Tokens** | Decentralized, flexible permission system (e.g., IoT, Web3 APIs) | Uses cryptographic proof with logic-based authorization | Portable, append-only structure for delegation | Complexity of managing policy-based access |

---

## **Comparison Table: Web2 vs Web3 Tokens & Keys**
| **Dimension**       | **Web2 (Traditional Authentication)** | **Web3 (Decentralized Authentication)** |
|---------------------|--------------------------------------|--------------------------------------|
| **Identity Management** | Managed by centralized providers (Google, Facebook, etc.) | Self-custodied (wallets, private keys) |
| **Access Control** | Based on username/password, OAuth, API keys | Based on cryptographic signatures (ECDSA, Ed25519) |
| **Security Model** | Depends on central servers & encrypted storage | Trustless, secured by cryptography |
| **Vulnerability** | Password leaks, session hijacking, phishing | Private key loss, phishing, social engineering |
| **Recovery Mechanism** | Password reset via email, MFA | Seed phrases (no central recovery option) |
| **Authorization** | OAuth, Bearer tokens, API keys | Wallet signatures, smart contract permissions, UCAN, Biscuit |
| **Where It’s Strong** | Works well for traditional web applications | Ideal for dApps, DeFi, and decentralized identity |
| **Where It’s Weak** | Prone to centralized breaches, privacy concerns | Harder UX, private key management is critical |

---

## **When Not to Use Each Approach**
| **Type** | **Not Useful When…** |
|---------|--------------------|
| **Web2 Session Tokens** | You need cross-domain authentication (SSO) without cookies |
| **OAuth/Bearer Tokens** | The system must be fully decentralized without third-party auth |
| **API Keys** | Security is critical (API keys are easily leaked if not well protected) |
| **Private Keys (Web3)** | Users are not comfortable managing their own security |
| **Seed Phrases** | A user-friendly login experience is needed (too complex for casual users) |
| **DID Keys** | You need wide adoption (DID standards are still evolving) |
| **UCAN** | You need immediate Web2 compatibility (adoption is still maturing) |
| **Biscuit Tokens** | Your system does not require fine-grained, logic-based authorization |

---

## **ACLs vs. Capability-Based Access Control**
| **Feature**            | **ACL (Access Control List)** | **Capability-Based Security** |
|------------------------|-----------------------------|------------------------------|
| **Definition**         | A list of permissions attached to an entity (user, role, or group) | A token or key that grants specific rights directly |
| **Access Control Basis** | Based on identities (user IDs, roles, or groups) | Based on possession of a valid capability (token, key, or object reference) |
| **Authorization Model** | Centralized: a server checks permissions per request | Decentralized: authority is delegated with cryptographic tokens |
| **Granularity**        | Role-based or user-based access | Fine-grained access tied to specific capabilities |
| **Example Systems**    | UNIX file permissions, AWS IAM policies, OAuth Scopes | UCAN (User-Controlled Authorization), Biscuit Tokens |
| **How Access is Granted** | User is checked against a list of permissions stored in a system | User presents a token that proves they have the necessary capability |
| **Delegation of Rights** | Harder, requires explicit rule changes | Easier, rights can be transferred securely via tokens |
| **Security Model**     | Central authority controls access (can be misconfigured) | Distributed trust model (reduces single points of failure) |
| **Revocation Complexity** | Requires modifying ACL rules in a central database | Capabilities can have built-in expiry or revocation lists |
| **Attack Surface**    | Vulnerable to ACL misconfigurations, insider attacks | Requires secure handling of tokens to prevent leaks |
| **Common Use Cases**   | Enterprise security, file system permissions, API access (OAuth) | Distributed systems, smart contracts, decentralized identity (DIDs) |
| **Flexibility**        | Less flexible, requires pre-configured roles | More flexible, can grant fine-grained and temporary access |

## **Key Takeaways**
- **ACLs**: Identity-based, centralized, common in traditional Web2 security.
- **Capabilities**: Token-based, decentralized, common in Web3, IoT, and distributed systems.
- **UCAN & Biscuit**: Examples of capability-based models enabling fine-grained, decentralized access control.


# References
## [1] Video Reference: "Decentralizing Auth, and UCAN Too" by Brooklyn Zelenka
- **Title**: [Decentralizing Auth, and UCAN Too](https://www.youtube.com/watch?v=MuHfrqw9gQA)
- **Speaker**: Brooklyn Zelenka
- **Topic**: This video discusses decentralizing authentication systems, particularly focusing on **UCAN (User-Controlled Authorization Networks)**, a decentralized alternative to traditional access control models.

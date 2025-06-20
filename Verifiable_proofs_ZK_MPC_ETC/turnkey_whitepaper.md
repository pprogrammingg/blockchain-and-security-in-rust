# Key Management Re-Imagined from Frist Principles

- Every transaction in Web3 starts and ends with private keys. Typically hard to store and manage.

## Meet users where they are

familiar login methods like passkeys, email , OAtuh ( leave passwords and secrets out)
Only store public keys. TEESs are used to perform exchange of keys and bridge traditional means (email, etc).
More details in architecture diagram. For machines use API Key exchange.

## Build on shared cryptography primitives

Instead of transactions and assets to abstract, use Curve-Level
operations and signing schemes as fundamentals.

Building on Curve level makes working with different blockchains very easy and seamless.
Bitcoin, Ethereum, and Cosmos sign with Secp256k1. Solana, Polkadot, Stellar, Sei and Sui all sign using Ed25519.
2 curves covers a lot, Turnkey has added 10.

To make it easier, Turnkey offers a tiered approach, with highest tier providing easiest way to integrate.

Tier 1: curve level
Tier 2: Address Derivation
Tier 3: Client side SDK to construct and sign transactions
Tier 4: Transaction parsing and defining policies based on transactions params

## Engineer for speed and scale

Payments, cross-chain, AI Agent interacting with crypto, lightning fast signing,
etc. Deliver sub-100 ms signing latency and scale to millions of wallets.
Private keys are run inside TEE with QoS (a minima Linux kernel) wtih TEE apps written in Rust. Very close to
hardware.100 ms including network etc. low overhead, synchronous operations

## Assume everything is compromised until proven secure

run sensitive ops inside "trusted and verifiable" space. The
entire perimeter of signing process is secured rather than just key material. All request to modify data is signed
by
user-held authenticator, verified and processed entirely within secure enclaves.

A lot of TEE solutions used services like AWS/Google KMS,SafeNet HSM, etc. where essentially a Linux server makes
a request to encrypt/decrypt something without context. An Attacker does not need to compromise the secure
systems, they just have to compromise the traditional service.

Attackers do not need to steal private keys, they can manipulate transaction parsing, malicious
authentication,policy evaluation, etc

Everything from key generation to signing, etc is run inside Trusted Computing Base (TCB). Outside TCB software
should not modify data, nor modify/trigger user requests

## Don't trust, verify

divide the space to "trusted" (where critical software runs) vs "untrusted". As an
industry-first, trusted space is verifiable with remote attestations. Security claims are fully tangible, real and
verifiable.

Assumes everything is compromised by default. Trust only components that are externally verified and trust in **quorum**
of operators to eliminate classic single-point-of-failure.

## Be pragmatic, not dogmatic

in the long run some aspects of this will be decentralized. Compared to ETH L2 eventual protocol change to use
passkeys (which will take long),
Turnkey supports PassKey auth across multiple chains. They hope to be leveraged as a lab for the crypto industry. Lots
of integrations already going e.g. Alchemy and ZeroDev

## Build a library not a Framework

a lib full of modular building blocks rather than a restrictive framework. It is
the Unix philosophy applied to key management.

- User use authenticator to work with Turnkeh
- Business use authenticator on behalf of end-user
- User and Business collaborate to authenticate using consensus features

Building blocks rather than all-encompassing framework can be used for Gnosis Safe, Bitcoin Multi-Sig,
can be used to deploy smart contracts, can be used to sponsor gas, bridge funds and safe gaurd domain names

# Verifiable Foundations

- TEEs can be used to verify using remote attestations -> using measurements available in
  Platform Configuration Registers
- Quorum OS is acting as a glue between provider-specific HW and provider-specific application software
- need to verify that enclave is running the correct QoS , and QoS running the right application binary
- Remote attestations require secure builds and StageX creates verifiable builds used widely at Turnkey
- StageX guarantees a 1-to-1, immutable relationship between human-readable source code and
  the resulting machine-executable artifacts running inside of QuorumOS.
- Boot-proofs, app-proofs + StageX + QoS + Remote Attestations yield full verifiability
- TEE and Nitro Enclave
    - Other than gauranteed isolations, have secure sources of time and entropy
    - Most interesting part is RA (remote attestation) / PCR measurements
- Overview
    - TEEs are paired with host, and protect it against OS-level vulnerability and malware
    - Many standards govern TEEs: JavaCard, FIPS, TPM, Global Platform, etc
    - Mobile Device Auth (iOS) and Trustee (Android) are most well-known usecases which is tied to
      fingerprint and FaceID
    -

# Turnkey's Architecture

# Applications Beyond Key Management


Source: https://docs.dwallet.io/

- Sub-second MPC network, can do 10K signatures per sec, support
  order of 100k signing nodes with zero-trust, noncollusive security
- dWallets are programmable, decentralized signing mechanisms that
  allow controling native assets in other chains from their own smart-
  contracts

# Unique Value

- Scalable decentralized MPC infra
- Sub-second speed, enabling cross-chain
- Transcends node limit of MPC network and can scale to 100s of nodes
  generating signatures
- Zero-Trust Security
- Cross-chain interactions even with Bitcoin is possible
- Supports `ECDSA` soon `EdDSA`
- Massive Decentralization: 2PC-MPC protocol scales to hundreds even
  thousands of signers in the signature process
- Native Interoperability: No bridging, no Wrapping....control native
  assets directly

# Cryptography of dWallets - 2PC-MPC

- dWallets use 2PC-MPC, two-party ECDSA protocol.
- The second party is fully emulated by a network of N parties.
- Linear scaling in communication (`O(n)`)
- Cost per party is `O(1)` for up to 1000s of parties - for user it is
  asymptotically `O(1)` - meaning size of the network has no impact on the user
  since communication and computation remain constant

# IKA Network Overview

- IKA is an MPC network forked from SUI
- Maintained by set of permissionless authorities set that play a role
  similar to validators/miners
- IKA Modified SUI by implementing 2PC-MPC using Sui;s consensus Mysticeti
  between nodes
- Composable modular signature network,
- To allow Smart Contract on another chain, to control a dWallet, state proofs
  for that chain must be available on IKA in the form of light clients
- SUI State proofs are first to be implemented, so builders can use dWallets
  as building blocks in their smart contracts.
- IKA has a coin launching on SUI as native token of IKA network
- IKA coin is used as a delegated stake on authorities like gas within an epoch
- Voting power in an epoch is the result of this delegated stake
- In any epoc, set of authorities is Byzantine fault-tolerant and collect and distribute fees for parcitipation
- SUI is backed by years of peer-reviewed and Opnesource dev




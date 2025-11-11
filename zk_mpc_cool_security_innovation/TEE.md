# Insights from https://quorum.tkhq.xyz/

## TLS from TEE Enclave

- Why:
    - There is no NIC in TEEs (no network interface), so cannot get a list of public keys over the network.
    - Public keys are needed to check the ODIC token against them, otherwise forged public keys will make it
    - Public keys are needed to be fetched as they are rotating too often (cannot be hardcoded)
    - They can not be provided at run-time which is against (Turnkey's) threat model.
    - There is no way to record a TLS session. Because of symmetric keys TLS does not provide non-repudiation (who
      sent and received what)

- Part 1: What is TEE and Nitro
    - Enclave is isolated VM with own CPU and memory (just volatile memory only so stateless)
    - Only communication interface is a VSock (there are no NICs)
    - Has access to independent source of time and entropy - this is Nitro Security Module (NSM)
    - On boot, enclave generates a pair of cryptographic keys called ephemeral keys
    - Can provide attestations containig measurements like Platform Config Registers, Boot Ram , etc
        - PCRs: where hash computations are formed using prev state of PCR vs next so PCR[0] = Hash(PCR[0] concatenate
          New Hash Value) - hash of (old concatenated to new hash)
          so hashes are commulative chains, the type of operation is referred to as `extending` the hash and result is
          called `measurement`
      
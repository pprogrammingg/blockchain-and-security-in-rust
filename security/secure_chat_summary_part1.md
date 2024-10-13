# Source of Tutoral
[Making a Secure Chat in Rust](https://vaktibabat.github.io/posts/Making_A_Secure_Chat_Rust_Crypto/)

# Overview
1. Prevent eavesdroppers from detecting chat content
2. Authenticate who we are chatting with

# Pre-requisties 
- [Final Fields](https://en.wikipedia.org/wiki/Finite_field)

# Chat Code
- function `peer_loop`
  - Given a TCP stream split it to reader and writer and 
  - Loop
    - Select of these Asyncs
      - if there are lines (`lines.next_line()`)
        - extract and parse the command 
        - Match command on `help`, `connect`, `leave`, `quit`, `listen` and `send`
          - `send` => `handle_send()` 
      
      - else read msg
        - print msg
  - return ok()
  
- `handle_connect(cmd)`
  - Given command, extract host and port
  - connect TCP with host/port and get TCP stream back
  - invoke `peer_loop` with stream
  - retunr Ok(()) once done

- `handle_send(cmd, tcp_stream_writer)`
  - Given command args, concatenate all with space to reduce TCP steam writes
  - Send the whole thing over to the writer

-`handle_listen(cmd)`
  - Given command, extract host and port
  - Bind TCP listener to host and port
  - Accept the listener to get the TCP stream
  - invoke `peer_loop` on the stream
  - exit with Ok(())

# Chat is Insecure
- Can use see all msgs in plain text using MITM attack or ARP poisoning 
- Uses Wireshark to demonstrate this

# Symmetric vs Asymmetric Cryptography
- Symmetric
  - Give a key to both users. Cipher <-> key <-> plaintext (2-way funciton)
  - challenge: how to transmit the key - attacker in between before securing channel
  can intercept (possible solutions: give key through OOB method like courier service or 
  use Asymmetric)
  - Most common is AES (128, 192 and 256-bit), key space is 2^256 almost as big as observable
  atoms in the universe

- Asymmetric
  - Each person hold a known public key and their own secret private key
  - Mostly based on one-way function- easy to compute cipher in one direction
  - RSA, Diffie-Hellman and elliptic curve discrete logarithm (ECDL) are most 
  common of this form. 

( plain text -> Encrypt <- Alice's public key ) ===> Cipher

Cipher -> Alice's Private Key ===> plaintext

Note: 
- Symmetric crypto is cheaper than asymmetric
- So usually use asymmetric key to encrypt the symmetric key that will be used for the next phases
of encrypting using symmetric key

1. Bob generates symmetric key
2. Bob uses asymmetric key to encrypt symmetric key called Symmetric_Key
3. Bob sends the encrypted key over to Alice
4. Alice using asymmetric private key A_PRA decrypts and gets Symmetric_Key
5. Alice and Bob exchange messages using Symmetric_Key

Another important capability of asymmetric key is signing messages.
e.g. send X amount from A to B. To validate that the message is indeed sent from the signer. 
In this case the transaction processor like Bank can use the sender public key to valdiate whether the 
message was signed by the sender private key.


# Secure Chat Architecture
1. See above steps for passing a symmetric key using asymmetric key and then using that key
for send/receive messages
2. One note is that Alice and Bob also need to exchange public keys. But attacker can instead pass
their own public key and Bob mistakenly encrypts using their public key which upon further interception
allows the attacker to intercept
Options here are : use a trusted third-party (TPP), or a key-store or send it through an OOB channel
3. in TPP, a digest of info including user pub key, name, address, etc. is shashed (a digest is what the 
output is called). Others verify the certificate againsdt TPP.

# TPP Code
1. Receives specific requests from users
2. for each task like `RequestCertificate` or `ValidateCertificate` spawn a new task and handle them
3. `RequestCertificate` name 
   - construct a digest using name and byte manipulations 
   - TPP signs the digest using MD5 (not secure)
4. `ValidateCertificate`
   - parses the certificate from message and validates the signature

- Note: Grok `RequestCeritificare` or `ValidateCertificate` payload construction 
(unit test as an exercise). 

# More on Ceritificates
- Certificates are used to secure all traffic on the internet using the
TLS (Transport Layer Security) protocol.
- To use TLS, websites apply for Certificate Signing Request (CSR) to a TPP
  (Certificate Authority or CA for short)
- In MITM attack, CA tells that certificate is in valid, the browser won't connect
unless we set it as exception.
- CA can be hacked in which attacker could sign certificates of their own
to personate websites and bypass browser warning.

# What we have so far
- Each user has a key pair
- User asks TPP to sign a certificate
- When users once to connect to each other, they send each other's certificate 
to the TPP
- RSA used to exchange a symmetric key secretly
- Symmetric key used to encrypt all messages using AES 

# Security Algorithms

- RSA
  - NIST recommends 2048, works based on a one-way function multiplying two
  large primes. The algorithm described above is called textbook RSA, and it is vulnerable 
  to many attacks (for example, it is deterministic, so ciphertexts can be distinguished). 
  In real RSA implementations, we also apply PKCS#1 padding, 
  which pads the message with extra data and randomness.
  - Fermat and Miller-Rabin (stronger test) tests to assert primality
  - Probabilistic vs accurate algorithms with efficiency trade-offs
  - Exercise: grok Rust code for RSA

- AES
  - A block cipher
  - Operates on block of data, instead of bit by bit
  - Side notes: ciphers that work on bits are called stream cipher
  - Block-size in AES is 128-bit
  - Block ciphers consist of rounds - many weak operations in each round
  but many in number. A round key dictates how the round will behave. Round keys are 
  derived from the main symmetric key K using an algorithm called the key schedule.
  - Possible key sizes 128-bits, 192-bits and 256-bits
  - Round key defines mode of operation, some common ones:
    - ECB - simple but fails to hide data patterns
    - CBC - each block XORed with previous block. First block XORed 
    with IV (initialization vector). CBC has more diffusion than ECB, better use in chat.
    - AES Crate (note had to pad blocks to get 16-byte blocks

# All of it Together
## Connect
- Connect to chat server perform handshake
- connect to TPP server
- Ask for chat server certificate 
- Validate certificate by sending it to TPP
- If not valid shutdown conn to server
- If valid, let server know certificate is accepted
- Expect server will send request for client's certificate
- Send client's cert to server, given the received request above
- Listen to see whether server accepts are cert
- if not exit
- At this point, server cert is validated by client and client's cert by the server
- Listen for server sending the symmetric key plus IV for CBC
- Client gets an encrypted version of symmetric key (encrypted by TPP using client public key pre-sent to them)
- Client decrypts the above to get the private key

## Server Listen to Client
- Listen 
  - Expect client ask for server (self)'s cert
  - send them the cert
  - Wait for their acceptance
  - Ask for their cert, if not valid shutdown everything
  - If valid, generate symmetric key and ecrypt it using client's pub key from cert
  - send the ecrypted key
  - store the streeam and cipher of the key with respect to client













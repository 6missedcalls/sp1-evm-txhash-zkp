# SP1 Ethereum Transaction Hash Proof (ZK Attestation Circuit)

> ⚠️ Proof-of-concept. Built quickly to demonstrate a minimal ZK primitive. Only supports legacy Ethereum transactions (type 0) as of now.

---

## 🔐 What Does This ZK Circuit Prove?

This project implements a zero-knowledge circuit using [SP1 zkVM](https://github.com/succinctlabs/sp1) to prove:

> “I know a raw RLP-encoded Ethereum transaction that hashes to this exact `Keccak256(tx)` hash.”

### ✅ In-ZK Circuit Behavior:
- Reads the raw RLP bytes
- Computes Keccak256
- Compares to the claimed hash
- Commits a single boolean: `true` if valid, `false` otherwise

**No addresses, chain names, values, or signatures are revealed or committed.**

---

## ✨ Real-World Use Cases

- **ZK Receipts** – Prove that a payment or interaction happened without revealing who, what, or when
- **Cross-chain Proofs** – Prove something occurred on Ethereum or Base and use it privately on another chain
- **Gated Access** – Show that a user interacted with a DAO, contract, or token before unlocking a resource
- **Private Onboarding** – Let users prove past on-chain behavior without disclosing addresses

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/docs/sp1/getting-started/install)

## What does this ZKP prove?

This project demonstrates a zero-knowledge proof (ZKP) system that proves the existence and validity of a specific Ethereum transaction, without revealing any private information or requiring trust in a third party. The ZKP circuit and its associated scripts are designed to:

1. **Take as input:**

   - The transaction hash (`hash`)
   - The chain name (`chain_name`)

2. **Fetch and process (off-chain, in the script):**

   - The script uses the provided transaction hash to fetch the full transaction details from an Ethereum node (via Infura and ethers-rs).
   - It reconstructs the RLP-encoded form of the transaction using the transaction fields (nonce, gas price, gas, to, value, input, v, r, s).
   - It computes the Keccak-256 hash of the RLP-encoded transaction, which is the canonical transaction hash on Ethereum.

3. **Prove, in zero-knowledge, that:**
   - The claimed transaction hash matches the Keccak-256 hash of the RLP-encoded transaction data.
   - This check is performed inside the zkVM circuit, and the result (valid/invalid) is committed as a public output.

### How the Script Works (script/src/bin/main.rs)

- **User Input:**

  - The user only needs to provide the transaction hash, chain name as command-line arguments.

- **Transaction Fetching:**

  - The script connects to an Ethereum node (via Infura) and fetches the transaction details using the provided hash.

- **RLP Encoding:**

  - The script reconstructs the RLP-encoded transaction by serializing the fields in the correct order (nonce, gas price, gas, to, value, input, v, r, s).
  - It appends the raw values of `v`, `r`, and `s` directly, as these are always present in a confirmed transaction.

- **Hash Calculation:**

  - The script computes the Keccak-256 hash of the RLP-encoded transaction bytes. This is the canonical transaction hash on Ethereum.

- **Input to zkVM:**

  - The script writes the length of the RLP bytes, the RLP bytes themselves, and the claimed hash to the zkVM input stream.

- **Proof Generation:**
  - The zkVM program reads these inputs, recomputes the Keccak-256 hash inside the circuit, and checks that it matches the claimed hash. It then commits a boolean indicating whether the check passed.

### End-to-End Flow

- The user provides a transaction hash, chain name, and chain id.
- The script fetches the transaction, reconstructs the RLP encoding, and computes the hash.
- The script passes these to the zkVM program.
- The zkVM program verifies the hash matches the RLP encoding, and outputs a proof of this fact.

This allows anyone to verify, in zero-knowledge, that a specific transaction hash corresponds to a valid RLP-encoded Ethereum transaction, without revealing any additional information or requiring trust in the prover.

---

## Running the Project

There are 3 main ways to run this project: execute a program, generate a core proof, and
generate an EVM-compatible proof.

### Build the Program

The program is automatically built through `script/build.rs` when the script is built.

### Execute the Program

To run the program without generating a proof:

```sh
cd script
cargo run --release -- \
  --execute \
  --hash 0xc416863b395d6c1d984d7a1cf9ab1bddb8f73d201efb943e24d15ce996842ace \
  --chain-name eth-mainnet
```

This will execute the program and display the output.

### Generate an SP1 Core Proof

To generate an SP1 [core proof](https://docs.succinct.xyz/docs/sp1/generating-proofs/proof-types#core-default) for your program:

```sh
cd script
cargo run --release -- --prove \
  --hash 0xc416863b395d6c1d984d7a1cf9ab1bddb8f73d201efb943e24d15ce996842ace \
  --chain-name eth-mainnet
```

### Generate an EVM-Compatible Proof

> [!WARNING]
> You will need at least 16GB RAM to generate a Groth16 or PLONK proof. View the [SP1 docs](https://docs.succinct.xyz/docs/sp1/getting-started/hardware-requirements#local-proving) for more information.

Generating a proof that is cheap to verify on the EVM (e.g. Groth16 or PLONK) is more intensive than generating a core proof.

To generate a Groth16 proof:

```sh
cd script
cargo run --release --bin evm -- --system groth16
```

To generate a PLONK proof:

```sh
cargo run --release --bin evm -- --system plonk
```

These commands will also generate fixtures that can be used to test the verification of SP1 proofs
inside Solidity.

### Retrieve the Verification Key

To retrieve your `programVKey` for your on-chain contract, run the following command in `script`:

```sh
cargo run --release --bin vkey
```

## Using the Prover Network

We highly recommend using the [Succinct Prover Network](https://docs.succinct.xyz/docs/network/introduction) for any non-trivial programs or benchmarking purposes. For more information, see the [key setup guide](https://docs.succinct.xyz/docs/network/developers/key-setup) to get started.

To get started, copy the example environment file:

```sh
cp .env.example .env
```

Then, set the `SP1_PROVER` environment variable to `network` and set the `NETWORK_PRIVATE_KEY`
environment variable to your whitelisted private key.

For example, to generate an EVM-compatible proof using the prover network, run the following
command:

```sh
SP1_PROVER=network NETWORK_PRIVATE_KEY=... cargo run --release --bin evm
```

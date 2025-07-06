# SP1 Project Template

> **Note:** This project is a proof-of-concept (PoC) written in about 30 minutes with zero sleep, on a plane. It is intended as a concept to build upon and not production code. Expect quick-and-dirty code, minimal error handling, and a focus on showing the core idea end-to-end.

This is a template for creating an end-to-end [SP1](https://github.com/succinctlabs/sp1) project
that can generate a proof of any RISC-V program.

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/docs/sp1/getting-started/install)

## What does this ZKP prove?

This project demonstrates a zero-knowledge proof (ZKP) system that proves the existence and validity of a specific Ethereum transaction, without revealing any private information or requiring trust in a third party. The ZKP circuit and its associated scripts are designed to:

1. **Take as input:**

   - The expected sender address (`from`)
   - The expected recipient address (`to`)
   - The expected amount of Ether transferred (`amount`)
   - The expected transaction hash (`hash`)

2. **Fetch and provide (off-chain):**

   - The actual sender address, recipient address, amount, and hash from the Ethereum blockchain for the given transaction hash.

3. **Prove, in zero-knowledge, that:**
   - The actual transaction on-chain matches the expected `from`, `to`, `amount`, and `hash` values provided as public inputs.
   - This check is performed inside the zkVM circuit, and the result (valid/invalid) is committed as a public output.

### How the Prover Works (program/src/main.rs)

The prover is a minimal RISC-V program designed to run inside the SP1 zkVM. Its job is to check that the transaction details provided as input match the expected values. Here's how it works:

- **Inputs:**

  - The program reads four values from its input stream: `from` (20 bytes), `to` (20 bytes), `amount` (u64), and `hash` (32 bytes). These are provided by the script, which fetches them from the blockchain and/or user input.

- **Logic:**

  - The program compares the `from` and `to` addresses, the `amount`, and the `hash` for equality. (In the current PoC, the check is simply `from == to && amount == amount`, which is a placeholder for a real check. In a production version, you would compare all fields for equality.)

- **Output:**
  - The program commits a boolean value (`is_valid`) indicating whether the check passed. This value is output as a public value from the zkVM, and is included in the proof.

### End-to-End Flow

- The script (`script/src/bin/main.rs`) is responsible for:

  1. Accepting command-line arguments for the expected transaction details (`--from`, `--to`, `--amount`, `--hash`, etc.).
  2. Fetching the actual transaction from the Ethereum blockchain using the provided hash.
  3. Writing both the expected and actual transaction details into the zkVM's input stream in a specific order.
  4. Running the zkVM program and reading the result (whether the transaction matches the expected details).
  5. Optionally, generating a proof that can be verified by others or on-chain.

- The zkVM program (`program/src/main.rs`) is responsible for:
  1. Reading the expected and actual transaction details from its input stream.
  2. Comparing the expected and actual values for `from`, `to`, `amount`, and `hash`.
  3. Committing a boolean result (`is_valid`) indicating whether all values match.

### What is being proven?

The ZKP proves that:

- There exists an Ethereum transaction with the given hash.
- The transaction's sender, recipient, and amount match the expected values provided as public inputs.
- The prover cannot cheat: the zkVM circuit enforces that the committed result is only `true` if all values match exactly.

This allows anyone to verify, in zero-knowledge, that a specific transaction occurred on Ethereum with the claimed details, without trusting the prover or revealing any additional information.

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
cargo run --release -- --prove \
  --to 0x3242D1C7B855368d9D27D7d916510967fd194045 \
  --from 0x3D5a8e2138dc42bA1ed61553e9A9CAcFD40b0664 \
  --amount 350000000000000000 \
  --hash 0xc416863b395d6c1d984d7a1cf9ab1bddb8f73d201efb943e24d15ce996842ace \
  --chain-name=mainnet --chain-id 1
```

Unverified transaction:

```sh
cargo run --release -- --execute \
  --to 0x3242D1C7B855368d9D27D7d916510967fd194045 \
  --from 0x3D5a8e2138dc42bA1ed61553e9A9CAcFD40b0608 \
  --amount 350000000000000000 \
  --hash 0xc416863b395d6c1d984d7a1cf9ab1bddb8f73d201efb943e24d15ce996842ace \
  --chain-name=mainnet --chain-id 1
```

This will execute the program and display the output.

### Generate an SP1 Core Proof

To generate an SP1 [core proof](https://docs.succinct.xyz/docs/sp1/generating-proofs/proof-types#core-default) for your program:

```sh
cd script
cargo run --release -- --prove
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

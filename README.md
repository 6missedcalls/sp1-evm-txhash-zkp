# SP1 Ethereum Transaction Hash Proof (ZK Attestation Circuit)

> ⚠️ Proof-of-concept. Built quickly to demonstrate a minimal ZK primitive. Only supports legacy Ethereum transactions (type 0) as of now.

---

## 🔐 What Does This ZK Circuit Prove?

This project implements a zero-knowledge circuit using [SP1 zkVM](https://github.com/succinctlabs/sp1) to prove:

> “I know a raw RLP-encoded Ethereum transaction that hashes to this exact `Keccak256(tx)` hash.”

### ✅ In-ZK Circuit Behavior:

- Reads the raw RLP bytes
- Computes Keccak-256
- Compares to the claimed hash
- Commits a single boolean: `true` if valid, `false` otherwise

**No addresses, chain names, values, or signatures are revealed or committed.**

---

## ✨ Real-World Use Cases

- **ZK Receipts** – Prove that a payment or interaction happened without revealing who, what, or when
- **Cross-chain Proofs** – Prove something occurred on Ethereum or Base and use it privately on another chain
- **Gated Access** – Show that a user interacted with a DAO, contract, or token before unlocking a resource
- **Private Onboarding** – Let users prove past on-chain behavior without disclosing addresses

---

## 🧰 Requirements

- [Rust](https://rustup.rs/)
- [SP1 CLI and environment](https://docs.succinct.xyz/docs/sp1/getting-started/install)

Create a `.env` file in the root directory with your Infura key:

```env
INFURA_API_KEY=your_infura_key_here


⸻

⚙️ What This ZKP System Does
	1.	Off-chain (host script):
	•	Accepts a transaction hash (--hash) and chain name (--chain-name)
	•	Fetches the full transaction from an Ethereum-compatible RPC (e.g. Infura)
	•	Manually reconstructs the transaction’s canonical RLP encoding (currently for type 0 only)
	•	Computes the Keccak-256 hash of that RLP
	•	Prepares those values for the ZK circuit
	2.	In-circuit (SP1 zkVM):
	•	Reads the RLP bytes and claimed hash
	•	Computes Keccak-256 inside the circuit
	•	Compares to the claimed hash
	•	Commits a single public bool (true or false)

🛡️ Note: This proof does not validate that the transaction was mined or accepted by the Ethereum protocol. It simply proves knowledge of a transaction that hashes to the given hash.

⸻

🔁 End-to-End Flow
	•	You provide a transaction hash, chain name, and .env key
	•	The script:
	•	Confirms the tx exists on-chain
	•	Reconstructs its RLP
	•	Hashes it
	•	Sends both into the zkVM
	•	The zkVM proves: “Keccak256(rlp) == hash”

⸻

🧪 How the Script Works

Path: script/src/bin/main.rs
	•	Parses CLI args: --execute or --prove
	•	Loads INFURA_API_KEY from .env
	•	Maps chain name to an RPC URL
	•	Fetches the transaction using ethers-rs
	•	RLP encodes the fields (currently: nonce, gas price, gas, to, value, input, v, r, s)
	•	Writes RLP length, RLP bytes, and hash to SP1 stdin

⸻

🚀 Running the Project

There are 3 main modes: execute, core proof, and EVM-compatible proof.

🧪 Execute (no proof)

cd script
cargo run --release -- \
  --execute \
  --hash 0xc416863b395d6c1d984d7a1cf9ab1bddb8f73d201efb943e24d15ce996842ace \
  --chain-name eth-mainnet

🧾 Generate SP1 Core Proof

cd script
cargo run --release -- \
  --prove \
  --hash 0xc416863b395d6c1d984d7a1cf9ab1bddb8f73d201efb943e24d15ce996842ace \
  --chain-name eth-mainnet

⸻

🧱 TODO (Next Steps)
	•	Add EIP-1559 (type 2) transaction support
	•	Consider support for access lists (EIP-2930)
	•	Harden edge cases with malformed RLP inputs

⸻

📜 License

MIT
```

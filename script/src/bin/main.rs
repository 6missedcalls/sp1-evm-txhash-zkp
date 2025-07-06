//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use clap::Parser;
use ethers::providers::Middleware;
use ethers::types::H256;
use ethers::utils::keccak256;
use rlp::RlpStream;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use std::convert::TryFrom;
use std::str::FromStr;

pub const TXN_VERIFIER_PROGRAM: &[u8] = include_elf!("txn-verifier-program");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    #[arg(long)]
    hash: String,

    #[arg(long)]
    chain_name: String,
}

fn main() {
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    let infura_api_key = dotenv::var("INFURA_API_KEY").expect("INFURA_API_KEY must be set");
    let args: Args = Args::parse();
    if args.execute == args.prove {
        eprintln!("Please provide either --execute or --prove");
        std::process::exit(1);
    }

    // Determine the correct Infura RPC URL based on chain_name
    let rpc_url = match args.chain_name.to_lowercase().as_str() {
        "eth-mainnet" => format!("https://mainnet.infura.io/v3/{}", infura_api_key),
        "eth-sepolia" => format!("https://sepolia.infura.io/v3/{}", infura_api_key),
        "base-mainnet" => format!("https://base-mainnet.infura.io/v3/{}", infura_api_key),
        "base-sepolia" => format!("https://base-sepolia.infura.io/v3/{}", infura_api_key),
        _ => {
            eprintln!("Unsupported chain name: {}", args.chain_name);
            std::process::exit(1);
        }
    };
    println!("RPC URL: {}", rpc_url);

    // setup the client
    let client = ProverClient::from_env();

    // fetch the transaction and prepare inputs
    let (rlp_bytes, claimed_hash) = fetch_rlp_and_hash(&rpc_url, &args.hash);

    // read the inputs
    let mut stdin = SP1Stdin::new();
    stdin.write(&rlp_bytes.len());
    for b in &rlp_bytes {
        stdin.write(b);
    }
    stdin.write(&claimed_hash);

    if args.execute {
        // execute the program
        let (mut output, report) = client.execute(TXN_VERIFIER_PROGRAM, &stdin).run().unwrap();

        // read the output
        let valid = output.read::<bool>();
        println!("valid: {:?}", valid);

        if !valid {
            eprintln!("Transaction is invalid");
            std::process::exit(1);
        } else {
            println!("Transaction is valid");
        }

        // record number of cycles executed
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(TXN_VERIFIER_PROGRAM);
        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof!");
        println!("Successfully verified proof!");
    }
}

fn fetch_rlp_and_hash(rpc_url: &str, hash: &str) -> (Vec<u8>, [u8; 32]) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let provider = ethers::providers::Provider::try_from(rpc_url).unwrap();
    let hash = H256::from_str(hash).unwrap();
    let tx = rt
        .block_on(provider.get_transaction(hash))
        .unwrap()
        .unwrap();

    // For legacy transactions, manually RLP encode the fields
    let mut stream = RlpStream::new();
    stream.begin_list(9);
    stream.append(&tx.nonce);
    stream.append(&tx.gas_price.unwrap_or_default());
    stream.append(&tx.gas);
    stream.append(&tx.to.unwrap_or_default());
    stream.append(&tx.value);
    stream.append(&tx.input.0);
    stream.append(&tx.v);
    stream.append(&tx.r);
    stream.append(&tx.s);
    let rlp_bytes = stream.out().to_vec();
    let claimed_hash = keccak256(&rlp_bytes);
    (rlp_bytes, claimed_hash)
}

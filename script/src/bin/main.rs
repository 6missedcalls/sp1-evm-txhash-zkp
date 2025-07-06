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
use ethers::{prelude::*, types::U256};
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
    to: String,

    #[arg(long)]
    from: String,

    #[arg(long)]
    amount: String,

    #[arg(long)]
    hash: String,

    #[arg(long)]
    chain_name: String,

    #[arg(long)]
    chain_id: u64,
}

fn main() {
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    let infura_api_key = dotenv::var("INFURA_API_KEY").expect("INFURA_API_KEY must be set");
    let rpc_url = format!("https://mainnet.infura.io/v3/{}", infura_api_key);
    println!("RPC URL: {}", rpc_url);

    let args: Args = Args::parse();
    if args.execute == args.prove {
        eprintln!("Please provide either --execute or --prove");
        std::process::exit(1);
    }

    // setup the client
    let client = ProverClient::from_env();

    // read the inputs
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.from);
    stdin.write(&args.to);
    stdin.write(&args.amount);
    stdin.write(&args.hash);
    stdin.write(&args.chain_name);
    stdin.write(&args.chain_id);

    if args.execute {
        // execute the program
        let (mut output, report) = client.execute(TXN_VERIFIER_PROGRAM, &stdin).run().unwrap();

        // read the output
        let valid = output.read::<bool>();
        println!("valid: {:?}", valid);

        // verify the transaction ourselves
        // we will call ethers and check
        verify_transaction(&rpc_url, args.to, args.from, args.amount, args.hash).unwrap();
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
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}

#[tokio::main]
async fn verify_transaction(
    rpc_url: &str,
    to: String,
    from: String,
    amount: String,
    hash: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let to_address = Address::from_str(&to)?;
    let from_address = Address::from_str(&from)?;
    let amount = U256::from_dec_str(&amount)?;
    let hash = H256::from_str(&hash)?;

    let tx = provider.get_transaction(hash).await?;
    let tx = tx.ok_or("Transaction not found")?;

    if tx.to.expect("Transaction to address is None") != to_address {
        println!("Expected to:   {:?}", to_address);
        println!(
            "Actual tx.to:  {:?}",
            tx.to.expect("Transaction to address is None")
        );
        return Err("Transaction to address does not match".into());
    }
    if tx.from != from_address {
        return Err("Transaction from address does not match".into());
    }
    if tx.value != amount {
        return Err("Transaction amount does not match".into());
    }
    if tx.hash != hash {
        return Err("Transaction hash does not match".into());
    }

    Ok(())
}

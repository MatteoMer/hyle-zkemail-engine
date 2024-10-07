mod db;
mod engine;

// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use engine::ChessEngine;
use hyle_contract::{HyleInput, HyleOutput};
use referee::{
    hyle::HyleNetwork,
    server::{EmailServer, ServerConfig},
};

// TODO: abstract this better
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let domain: String = std::env::var("REFEREE_IMAP_DOMAIN").unwrap();
    let username: String = std::env::var("REFEREE_IMAP_USERNAME").unwrap();
    let password: String = std::env::var("REFEREE_IMAP_PASSWORD").unwrap();
    let port: u16 = std::env::var("REFEREE_IMAP_PORT").unwrap().parse().unwrap();
    let mut engine =
        ChessEngine::new(HyleNetwork::Devnet, username.as_ref(), password.as_ref()).await?;
    let mut server = EmailServer::new(
        &mut engine,
        "CheckmateVerifierV2",
        &domain,
        port,
        &username,
        &password,
    );
    server.run().await;

    Ok(())

    /*
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    /* RISC0 Proving */

    // TODO: get PGN from the mail
    // the input here is the PGN of the game

    let null_state = 0u32.to_be_bytes().to_vec();
    let env = ExecutorEnv::builder()
        .write(&)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let proof_info = prover.prove(env, CHESS_GUEST_ELF).unwrap();

    let receipt = proof_info.receipt;

    // extract the receipt.

    // TODO: think about what the journal should have

    let is_checkmate: bool = receipt.journal.decode::<bool>().unwrap();

    let hyle_output = HyleOutput {
        version: 1,
        index: 0,
        identity: "".to_string(),
        tx_hash: "343C2AAF6EC57BBB491C6E50C5FEB4D487BFD8AA81671803966075843E71DABD".into(),
        program_outputs: is_checkmate,
        payloads: pgn.clone().as_bytes().to_vec(),
        success: is_checkmate,
        initial_state: null_state.clone(),
        next_state: null_state.clone(),
    };

    println!("program output: {:?}", is_checkmate);

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    receipt.verify(CHESS_GUEST_ID).unwrap();

    let claim = receipt.inner.claim().unwrap().value().unwrap().pre;
    let receipt_json = serde_json::to_string(&receipt).unwrap();
    std::fs::write("proof.json", receipt_json).unwrap();

    let initial_state_b64 = BASE64_STANDARD.encode(&hyle_output.initial_state);
    let next_state_b64 = BASE64_STANDARD.encode(&hyle_output.next_state);
    let initial_state_u32 = u32::from_be_bytes(hyle_output.initial_state.try_into().unwrap());
    let next_state_u32 = u32::from_be_bytes(hyle_output.next_state.try_into().unwrap());
    let program_outputs = hyle_output.program_outputs;

    println!("{}", "-".repeat(20));
    println!("Method ID: {:?} (hex)", claim.digest().to_string());
    println!("chess guest id: {:?}", CHESS_GUEST_ID);

    println!(
        "proof.json written, transition from {} ({}) to {} ({})",
        initial_state_b64, initial_state_u32, next_state_b64, next_state_u32
    );
    println!("Program outputted {:?}", program_outputs);
    println!(
        "Payload hash {:?}",
        BASE64_STANDARD.encode(hyle_output.payloads)
    );
    */
}

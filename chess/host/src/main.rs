// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use base64::prelude::*;
use hyle_contract::{HyleInput, HyleOutput};
use methods::{CHESS_GUEST_ELF, CHESS_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

// TODO: abstract this better
fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    /* RISC0 Proving */

    // TODO: get PGN from the mail
    // the input here is the PGN of the game

    let null_state = 0u32.to_be_bytes().to_vec();
    let pgn: String = "1. e4 e6 2. d4 d5 3. e5 c5 4. c3 Nc6 5. Bb5 Bd7 6. Bxc6 Bxc6 7. a3 Ne7 8. h4 Nf5 9. g4 Nxh4 10. g5 Nf5 11. f4 g6 12. b4 cxd4 13. cxd4 Qb6 14. Bb2 a6 15. Nf3 a5 16. O-O axb4 17. Qd3 Bb5 18. Qb3 Bxf1 19. Kxf1 Qb5+ 20. Kg2 Qc4 21. Qxc4 dxc4 22. a4 c3 23. Nxc3 bxc3 24. Bxc3 Ne3+ 25. Kf2 Nc2 26. Ra2 Nxd4 27. Nxd4 Bc5 28. Kf3 Rd8 29. Nb5 Rd3+ 30. Ke2 Re3+ 31. Kf1 Rf3+ 32. Kg2 Rxf4 33. Ba5 Rf2+ 34. Rxf2 Bxf2 35. Kxf2 O-O 36. Nd6 Rd8 37. Nxb7 Rd4 38. Nc5 Rc4 39. Bb6 Rg4 40. Nd7 Rxg5 41. Nf6+ Kg7 42. Bc7 Rg4 43. a5 Ra4 44. Ke3 h5 45. Ne8+ Kf8 46. Nd6 h4 47. Nb7 g5 48. Bd6+ Ke8 49. Kf3 g4+ 50. Kg2 h3+ 51. Kg3 Ra2 52. Kxg4 h2 53. Bc7 h1=Q 54. Nd6+ Kd7 55. Bb6 Qg2+ 56. Kh5 Qg6+ 57. Kh4 Rh2# 0-1".to_string();
    let env = ExecutorEnv::builder()
        .write(&HyleInput {
            initial_state: null_state.clone(),
            identity: "".to_string(), //TODO: what should i put?
            tx_hash: vec![1],
            program_inputs: pgn.clone(),
        })
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let prove_info = prover.prove(env, CHESS_GUEST_ELF).unwrap();

    // extract the receipt.
    let receipt = prove_info.receipt;

    // TODO: think about what the journal should have

    let is_checkmate: bool = receipt.journal.decode::<bool>().unwrap();

    let hyle_output = HyleOutput {
        version: 1,
        index: 0,
        identity: "".to_string(),
        tx_hash: vec![1],
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

    let claim = receipt.inner.claim().unwrap();
    let receipt_json = serde_json::to_string(&receipt).unwrap();
    std::fs::write("proof.json", receipt_json).unwrap();

    let initial_state_b64 = BASE64_STANDARD.encode(&hyle_output.initial_state);
    let next_state_b64 = BASE64_STANDARD.encode(&hyle_output.next_state);
    let initial_state_u32 = u32::from_be_bytes(hyle_output.initial_state.try_into().unwrap());
    let next_state_u32 = u32::from_be_bytes(hyle_output.next_state.try_into().unwrap());
    let program_outputs = hyle_output.program_outputs;

    println!("{}", "-".repeat(20));
    println!(
        "Method ID: {:?} (hex)",
        claim
            .value()
            .unwrap()
            .pre
            .value()
            .unwrap()
            .merkle_root
            .to_string()
    );
    println!(
        "proof.json written, transition from {} ({}) to {} ({})",
        initial_state_b64, initial_state_u32, next_state_b64, next_state_u32
    );
    println!("Program outputted {:?}", program_outputs);
    println!(
        "Payload hash {:?}",
        BASE64_STANDARD.encode(hyle_output.payloads)
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899".to_string(),
        CommitmentConfig::confirmed(),
    );

    let home = home_dir().ok_or("Could not locate home directory")?;
    let keypair_path = home.join(".config/solana/id.json");

    let payer = read_keypair_file(&keypair_path)
        .map_err(|e| format!("Failed to read keypair from {:?}: {}", keypair_path, e))?;

    println!("Payer: {}", payer.pubkey());
}

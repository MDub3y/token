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

    let mint = Keypair::new();
    println!("Mint: {}", mint.pubkey());

    let name = "only possible on solana".to_string();
    let symbol = "OPOS".to_string();
    let uri = "https://c8.alamy.com/comp/3F058AT/spain-champions-2026-fifa-world-cup-2026-soccer-tournament-logo-with-castle-emblem-tshirt-tee-3F058AT.jpg".to_string();

    let metadata = TokenMetadata {
        name: name.clone(),
        symbol: symbol.clone(),
        uri: uri.clone(),
        update_authority: Some(payer.pubkey()).try_into()?,
        mint: mint.pubkey(),
        additional_metadata: vec![("a".to_string(), "b".to_string())],
    };

    let mint_space =
        ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::MetadataPointer])?;
    let metadata_space = metadata.get_packed_len()?;
    let total_space = mint_space + metadata_space;
}

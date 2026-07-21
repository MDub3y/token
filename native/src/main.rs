use home::home_dir;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, read_keypair_file},
    system_instruction,
};
use spl_token_2022::{extension::ExtensionType, state::Mint};
use spl_token_metadata_interface::state::TokenMetadata;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899".to_string(),
        CommitmentConfig::confirmed(),
    );

    let home = home_dir().ok_or("Could not locate home directory")?;
    let keypair_path = home.join(".config/solana/id.json");

    let payer = read_keypair_file(&keypair_path)
        .map_err(|e| format!("Failed to read keypair from {:?}: {}", keypair_path, e))?;

    println!("Payer: {}", payer.pubkey);

    let mint = Keypair::new();
    println!("Mint: {}", mint.pubkey);

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

    let lamports = connection.get_minimum_balance_for_rent_exemption(total_space)?;

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        lamports,
        total_space as u64,
        &spl_token_2022::id(),
    );
}

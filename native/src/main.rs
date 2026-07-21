use home::home_dir;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, read_keypair_file},
    signer::Signer,
    system_instruction,
    transaction::Transaction,
};
use spl_token_2022::{
    extension::{ExtensionType, metadata_pointer},
    state::Mint,
};
use spl_token_metadata_interface::{
    instruction::{initialize as init_metadata, update_field},
    state::{Field, TokenMetadata},
};
use spl_type_length_value::variable_len_pack::VariableLenPack;

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

    let lamports = connection.get_minimum_balance_for_rent_exemption(total_space)?;

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        lamports,
        total_space as u64,
        &spl_token_2022::id(),
    );

    let init_metadata_pointer_ix = metadata_pointer::instruction::initialize(
        &spl_token_2022::id(),
        &mint.pubkey(),
        Some(payer.pubkey()),
        Some(mint.pubkey()), // Points to itself
    )?;

    let init_mint_ix = spl_token_2022::instruction::initialize_mint2(
        &spl_token_2022::id(),
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        2,
    )?;

    let init_metadata_ix = init_metadata(
        &spl_token_2022::id(),
        &mint.pubkey(),
        &payer.pubkey(),
        &mint.pubkey(),
        &payer.pubkey(),
        name,
        symbol,
        uri,
    );

    let update_metadata_field_ix = update_field(
        &spl_token_2022::id(),
        &mint.pubkey(),
        &payer.pubkey(),
        Field::Key("a".to_string()),
        "b".to_string(),
    );

    let recent_blockhash = connection.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[
            create_account_ix,
            init_metadata_pointer_ix,
            init_mint_ix,
            init_metadata_ix,
            update_metadata_field_ix,
        ],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        recent_blockhash,
    );

    let signature = connection.send_and_confirm_transaction(&transaction)?;
    println!("Signature: {}", signature);

    let account_data = connection.get_account_data(&mint.pubkey())?;
    println!(
        "Mint Account successfully created! Data size: {} bytes",
        account_data.len()
    );

    Ok(())
}

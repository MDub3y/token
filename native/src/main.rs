fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899".to_string(),
        CommitmentConfig::confirmed(),
    );
}

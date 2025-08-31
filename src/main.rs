use shred_stream::generated::{client::ShredstreamClient, shredstream::CommitmentLevel};




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = "https://shreds-fra6-1.erpc.global";
    let mut client = ShredstreamClient::connect(&endpoint).await?;

    // The filter is experimental
    let request = ShredstreamClient::create_entries_request_for_accounts(
        vec![],
        vec![],
        vec![],
        Some(CommitmentLevel::Processed),
    );

    let mut stream = client.subscribe_entries(request).await?;
   
    while let Some(slot_entry) = stream.message().await.unwrap() {
        let entries =
            match bincode::deserialize::<Vec<solana_entry::entry::Entry>>(&slot_entry.entries) {
                Ok(e) => e,
                Err(e) => {
                    println!("Deserialization failed with err: {e}");
                    continue;
                }
            };
        println!(
            "slot {}, entries: {}, transactions: {}",
            slot_entry.slot,
            entries.len(),
            entries.iter().map(|e| e.transactions.len()).sum::<usize>()
        );
    }

    Ok(())
}

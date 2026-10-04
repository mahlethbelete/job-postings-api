use std::path::Path;

use job_postings_api::{routes, store::PostingStore};
use tokio::net::TcpListener;

const DATA_PATH: &str = "data/job_postings.json";
const ADDRESS: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = PostingStore::from_json_file(Path::new(DATA_PATH))?;
    println!("Loaded {} postings", store.all().len());

    let listener = TcpListener::bind(ADDRESS).await?;
    println!("Listening on http://{ADDRESS}");

    axum::serve(listener, routes::router(store)).await?;
    Ok(())
}

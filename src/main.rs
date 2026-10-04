use std::path::Path;

use job_postings_api::store::PostingStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = PostingStore::from_json_file(Path::new("data/job_postings.json"))?;
    println!("Loaded {} postings", store.all().len());
    Ok(())
}

use std::env;
use std::sync::OnceLock;
use supabase_storage_rs::errors::Error;
use supabase_storage_rs::models::{Bucket, StorageClient};

static STORAGE_CLIENT: OnceLock<StorageClient> = OnceLock::new();

fn get_storage_client() -> &'static StorageClient {
    STORAGE_CLIENT.get_or_init(|| {
        let url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
        let key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");
        StorageClient::new(url, key)
    })
}

async fn get_bucket() -> Bucket {
    let bucket = get_storage_client().get_bucket("guessfreak").await;
    match bucket {
        Ok(bucket) => bucket,
        Err(e) => panic!("Error getting bucket: {:?}", e),
    }
}

pub async fn download_file(path: &str) -> Result<Vec<u8>, Error> {
    let bucket = get_bucket().await;
    get_storage_client().download_file(&bucket.id, path, None).await
}


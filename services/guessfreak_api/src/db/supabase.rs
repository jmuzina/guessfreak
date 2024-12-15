use std::env;
use std::sync::OnceLock;
use supabase_rs::SupabaseClient;

static DB_CLIENT: OnceLock<SupabaseClient> = OnceLock::new();

pub fn get_db_client() -> &'static SupabaseClient {
    DB_CLIENT.get_or_init(|| {
        let url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
        let key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");
        SupabaseClient::new(url, key).unwrap()
    })
}

pub mod storage {
    use super::env;
    use std::cmp::min;
    use std::sync::OnceLock;
    use supabase_storage_rs::errors::Error;
    use supabase_storage_rs::models::{Bucket, FileObject, FileSearchOptions, StorageClient};

    static STORAGE_CLIENT: OnceLock<StorageClient> = OnceLock::new();
    static MAX_FILES_PER_PAGE: OnceLock<u32> = OnceLock::new();
    static MAX_SIGNED_URL_EXPIRY_TIME: OnceLock<u64> = OnceLock::new();

    fn get_max_files_per_page() -> u32 {
        *MAX_FILES_PER_PAGE.get_or_init(|| {
            env::var("MAX_FILES_PER_PAGE")
                .map(|s| s.parse::<u32>().unwrap_or(100))
                .unwrap_or(100)
        })
    }

    fn get_max_signed_url_expiry_time() -> u64 {
        *MAX_SIGNED_URL_EXPIRY_TIME.get_or_init(|| {
            env::var("MAX_SIGNED_URL_EXPIRY_TIME")
                .map(|s| s.parse::<u64>().unwrap_or(3600))
                .unwrap_or(3600)
        })
    }

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

    pub async fn get_files<'a>(path: &str, input_options: Option<FileSearchOptions<'a>>) -> Result<Vec<FileObject>, Error> {
        let bucket = get_bucket().await;
        let mut options: FileSearchOptions = input_options.unwrap_or_default();

        options.limit = Some(min(options.limit.unwrap_or(get_max_files_per_page()), get_max_files_per_page()));
        options.offset = Some(options.offset.unwrap_or(0));

        get_storage_client()
            .list_files(&bucket.id, Option::from(path), Option::from(options)).await
    }

    pub async fn get_file_url(path: &str, expires_in: Option<u64>) -> Result<String, Error> {
        let bucket = get_bucket().await;
        get_storage_client()
            .create_signed_url(
                &bucket.id,
                path,
                min(
                    expires_in.unwrap_or(get_max_signed_url_expiry_time()),
                    get_max_signed_url_expiry_time()
                )
            ).await
    }

    pub async fn get_file(path: &str) -> Result<Vec<u8>, Error> {
        let bucket = get_bucket().await;
        get_storage_client().download_file(&bucket.id, path, None).await
    }
}

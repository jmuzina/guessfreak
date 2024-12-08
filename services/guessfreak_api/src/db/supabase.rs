use std::env;
use std::sync::OnceLock;
use supabase_rs::SupabaseClient;
use supabase_storage_rs::models::StorageClient;

static DB_CLIENT: OnceLock<SupabaseClient> = OnceLock::new();
static STORAGE_CLIENT: OnceLock<StorageClient> = OnceLock::new();

pub fn get_db() -> &'static SupabaseClient {
    DB_CLIENT.get_or_init(|| {
        let url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
        let key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");
        SupabaseClient::new(url, key).unwrap()
    })
}

pub fn get_storage() -> &'static StorageClient {
    STORAGE_CLIENT.get_or_init(|| {
        let url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
        let key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");
        StorageClient::new(url, key)
    })
}

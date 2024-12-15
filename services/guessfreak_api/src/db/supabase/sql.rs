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
use std::sync::OnceLock;
use postgrest::Postgrest;

static DB_CLIENT: OnceLock<Postgrest> = OnceLock::new();

pub fn get_db_client() -> &'static Postgrest {
    DB_CLIENT.get_or_init(|| {
        Postgrest::new(
            dotenv::var("SUPABASE_REST_URL")
                .expect("SUPABASE_REST_URL must be set")
        )
            .insert_header(
                "apiKey",
                dotenv::var("ANON_KEY")
                    .expect("ANON_KEY must be set")
            )
            .insert_header(
                "Authorization",
                format!(
                    "Bearer {}",
                    dotenv::var("SUPABASE_KEY")
                        .expect("SUPABASE_KEY must be set")
                )
            )
    })
}
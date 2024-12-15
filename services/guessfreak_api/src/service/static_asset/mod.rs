use crate::db::supabase::get_db_client;
use crate::model::static_asset::StaticAsset;

pub async fn get_static_asset(id: u64) -> Option<StaticAsset> {
    let result = get_db_client()
        .from("static_asset")
        .eq("id", id.to_string().as_str())
        .execute()
        .await;

    match result {
        Ok(records) => {
            if records.is_empty() {
               None
            } else {
                serde_json::from_value(records[0].clone()).ok()
            }
        },
        Err(_) => None
    }
}

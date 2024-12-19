use crate::db::supabase::sql::get_db_client;
use crate::model::static_asset::StaticAsset;

pub async fn get_static_asset(id: u64) -> Result<StaticAsset, String> {
    serde_json::from_str::<StaticAsset>(
        &get_db_client()
            .from("static_asset")
            .select("id,created_at,asset_type,path,description")
            .eq("id", id.to_string())
            .single()
            .execute()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())
}
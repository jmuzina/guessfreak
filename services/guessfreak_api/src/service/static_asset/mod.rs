use crate::db::supabase::get_db_client;
use crate::model::static_asset::StaticAsset;
use crate::util::records::get_first_record_from_result_vec;

pub async fn get_static_asset(id: u64) -> Option<StaticAsset> {
    let records = get_db_client()
        .from("static_asset")
        .eq("id", id.to_string().as_str())
        .execute()
        .await;

    get_first_record_from_result_vec::<StaticAsset, String>(records)
}

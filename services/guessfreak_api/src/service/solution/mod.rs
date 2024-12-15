use crate::db::supabase::get_db_client;
use crate::model::solution::SolutionChance;
use crate::util::records::{get_first_record_from_result_vec};

pub async fn get_solution_chance_by_id(id: u64) -> Option<SolutionChance> {
    let records = get_db_client()
        .from("chance")
        .columns(vec![
            "id",
            "created_at",
            "solution(id, created_at)",
            "text_html",
            "static_asset(id, created_at, asset_type, path, description)",
            "chance_order"
        ])
        .eq("id", id.to_string().as_str())
        .execute()
        .await;

    log::info!("Got records: {:?}", records);

    get_first_record_from_result_vec(records)
}
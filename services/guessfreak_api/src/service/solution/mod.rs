use crate::db::supabase::get_db_client;
use crate::model::solution::SolutionChance;
use crate::util::records;
use crate::util::records::{get_first_record_from_result_vec, get_first_record_from_vec};

pub async fn get_solution_chance_by_id(id: u64) -> Option<SolutionChance> {
    let records = get_db_client()
        .from("chance")
        .columns(vec![
            "id",
            "created_at",
            "solution()",
            "text_html",
            "static_asset()",
            "chance_order"
        ])
        .eq("id", id.to_string().as_str())
        .execute()
        .await;

    get_first_record_from_result_vec(records)
}
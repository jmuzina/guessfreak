use crate::db::supabase::sql::get_db_client;
use crate::model::solution::{SolutionChance, SolutionGuess};

pub async fn get_solution_chance_by_solution_id_and_position(solution_id: u64, idx: usize) -> Result<SolutionChance, String> {
    serde_json::from_str::<SolutionChance>(
        &get_db_client()
            .from("chance")
            .select("id,created_at,text_html,static_asset(id,created_at,asset_type,path,description)")
            .range(idx, idx)
            .single()
            .order("chance_order.asc.nullslast,text_html.asc.nullslast")
            .eq("solution_id", solution_id.to_string())
            .execute()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?
    )
        .map_err(|e| e.to_string())
}


pub async fn guess_solution(solution_guess: &SolutionGuess) -> Result<bool, String> {
    get_db_client()
        .from(format!("{}_solutions", solution_guess.solution_type_name).as_str())
        .select("video_game!inner(name)")
        .eq("solution_id", solution_guess.solution_id.to_string().as_str())
        .eq(format!("{}.name", solution_guess.solution_type_name).as_str(), solution_guess.guess.as_str())
        .limit(1)
        .execute()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())
        .and_then(|text| {
            serde_json::from_str::<Vec<serde_json::Value>>(&text)
                .map_err(|e| e.to_string())
        })
        .map(|records| !records.is_empty())
}
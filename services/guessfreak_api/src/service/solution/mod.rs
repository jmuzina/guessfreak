use crate::db::supabase::sql::get_db_client;
use crate::model::solution::{Solution, SolutionChance, SolutionGuess};
use crate::util::records::{get_first_record_from_result_vec};

pub async fn get_solution_by_id(id: u64) -> Option<Solution> {
    let records = get_db_client()
        .from("solution")
        .columns(vec![
            "id",
            "created_at",
            "solution_type(id, name, label)",
        ])
        .eq("id", id.to_string().as_str())
        .execute()
        .await;

    get_first_record_from_result_vec(records)
}

pub async fn get_solution_chance_by_id(id: u64) -> Option<SolutionChance> {
    let records = get_db_client()
        .from("chance")
        .columns(vec![
            "id",
            "created_at",
            "solution(id, created_at, solution_type(id, name, label))",
            "text_html",
            "static_asset(id, created_at, asset_type, path, description)"
        ])
        .eq("id", id.to_string().as_str())
        .execute()
        .await;

    get_first_record_from_result_vec(records)
}

pub async fn guess_solution(solution_guess: &SolutionGuess) -> bool {
    let solution = get_solution_by_id(solution_guess.solution_id).await;
    match solution {
        Some(solution) => {
            let solution_type = solution.solution_type;

            log::info!("Doing guess for ID of type: {}_solutions and guess str: {}", solution_type.name, solution_guess.guess);

            false
            // this could be extended here to add more solution domain-specific stuff
        },
        None => {
            false
        }
    }
}
//
// pub async fn submit_solution_chance_guess(id: u64, guess: String) -> Result<(), String> {
//     let records = get_db_client()
//         .from("chance_guess")
//         .insert(vec![
//             ("chance_id", id.to_string().as_str()),
//             ("guess", guess.as_str())
//         ])
//         .execute()
//         .await;
// }
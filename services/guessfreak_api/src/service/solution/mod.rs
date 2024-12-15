use crate::db::supabase::get_db_client;
use crate::model::solution::{Solution, SolutionChance};
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

pub async fn get_solution_domain(id: u64) {
    let solution = get_solution_by_id(id).await;
    match solution {
        Some(solution) => {
            let solution_type = solution.solution_type;

            let other_stuff = get_db_client()
                .from(format!("{}_solutions", solution_type.name).as_str())
                .eq("solution_id", solution.id.to_string().as_str())
                .columns(vec![format!("{}(name)", solution_type.name).as_str()])
                .execute()
                .await;
        },
        None => {
            println!("No solution found with id: {}", id);
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
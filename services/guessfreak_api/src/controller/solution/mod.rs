use serde_json;
use crate::model::solution::SolutionGuess;
use crate::service::solution;
use crate::service::solution::get_solution_chance_by_id;

/**
 * @param id The id of the solution
 * TODO if i really want to keep this, there should be a solution service instead of doing this in controller
 * @return The solution
*/
pub async fn cnt_get_solution_by_id(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let solution_record = solution::get_solution_by_id(id).await;
    match solution_record {
        Some(solution) => Ok(warp::reply::json(&solution)),
        None => Err(warp::reject::not_found())
    }
}

pub async fn cnt_get_solution_chance(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    match get_solution_chance_by_id(id).await {
        Some(solution_chance) => Ok(warp::reply::json(&solution_chance)),
        None => Err(warp::reject::not_found())
    }
}

pub async fn cnt_guess_solution(solution_guess: SolutionGuess) -> Result<impl warp::Reply, warp::Rejection> {
    let result = solution::guess_solution(&solution_guess).await;
    Ok(warp::reply::json(&serde_json::json!({
        "result": result
    })))
}

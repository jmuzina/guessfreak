use serde_json;
use crate::model::solution::SolutionGuess;
use crate::service::solution;
use crate::service::solution::get_solution_chance_by_id;

pub async fn cnt_get_solution_by_id(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let solution_record = solution::get_solution_by_id(id).await;
    match solution_record {
        Ok(solution) => Ok(warp::reply::json(&solution)),
        _ => Err(warp::reject::not_found())
    }
}

pub async fn cnt_get_solution_chance(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    match get_solution_chance_by_id(id).await {
        Ok(solution_chance) => Ok(warp::reply::json(&solution_chance)),
        _ => Err(warp::reject::not_found())
    }
}

pub async fn cnt_guess_solution(solution_guess: SolutionGuess) -> Result<impl warp::Reply, warp::Rejection> {
    match solution::guess_solution(&solution_guess).await {
        Ok(result) => Ok(warp::reply::json(&serde_json::json!({
            "result": result
        }))),
        _ => Err(warp::reject::not_found())
    }
}

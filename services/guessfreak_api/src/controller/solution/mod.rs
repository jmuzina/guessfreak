use serde_json;
use crate::model::form::paging::PagedRequest;
use crate::model::solution::SolutionGuess;
use crate::service::solution::{get_solution_by_id, get_solution_chance_by_solution_id_and_position, guess_solution};

pub async fn cnt_get_solution_by_id(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    get_solution_by_id(id).await
        .map(|solution_record| {
            warp::reply::json(&solution_record)
        })
        .map_err(|_| warp::reject::not_found())
}

pub async fn cnt_get_solution_chance_by_solution_id_and_position(chance_id: u64, paging: PagedRequest) -> Result<impl warp::Reply, warp::Rejection> {
    get_solution_chance_by_solution_id_and_position(chance_id, paging).await
        .map(|solution_chance_records| {
            warp::reply::json(&solution_chance_records)
        })
        .map_err(|_| warp::reject::not_found())
}

pub async fn cnt_guess_solution(solution_guess: SolutionGuess) -> Result<impl warp::Reply, warp::Rejection> {
    guess_solution(&solution_guess).await
        .map(|result| {
            warp::reply::json(&serde_json::json!({
                "correct": result
            }))
        })
        .map_err(|_| warp::reject::not_found())
}

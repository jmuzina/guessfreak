use warp::Filter;
use crate::controller::solution;
use crate::model::solution::SolutionGuess;

/**
 * Solution routes
*/
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("solution")
        .and(
            get_solution_by_id()
                .or(get_solution_chance_by_id())
                .or(guess_solution())
        )
}

/**
 * Get a solution by its id
 * This is not really a useful endpoint, but it's a good place to start learning Warp :)
 * GET /solution/{id}
*/
fn get_solution_by_id() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::param::<u64>()
        .and(warp::get())
        .and_then(solution::cnt_get_solution_by_id)
}

fn get_solution_chance_by_id() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("chance")
        .and(warp::path::param::<u64>())
        .and(warp::get())
        .and_then(solution::cnt_get_solution_chance)
}

fn guess_solution() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("guess")
        .and(warp::post())
        .and(warp::body::form::<SolutionGuess>())
        .and_then(solution::cnt_guess_solution)
}


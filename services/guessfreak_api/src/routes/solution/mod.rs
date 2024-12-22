use warp::Filter;
use crate::controller::solution;
use crate::model::solution::SolutionGuess;

/**
 * Solution routes
*/
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("solution")
        .and(
            get_solution_chance_by_solution_id_and_position()
                .or(guess_solution())
        )
}

fn get_solution_chance_by_solution_id_and_position() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("chance")
        .and(warp::get())
        .and(warp::path::param::<u64>())
        .and(warp::path::param::<usize>())
        .and_then(solution::cnt_get_solution_chance_by_solution_id_and_position)
}

fn guess_solution() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("guess")
        .and(warp::post())
        .and(warp::body::form::<SolutionGuess>())
        .and_then(solution::cnt_guess_solution)
}


use warp::Filter;
use super::super::controller::solution;

// A function to build our routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_post()
}

// A route to handle GET requests for a specific post
fn get_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("solutions" / u64)
        .and(warp::get())
        .and_then(solution::get_solution)
}
use warp::Filter;
use super::solution::solution;

/**
    * All API routes
*/
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    solution::routes()
    //.or(other::routes())..... TODO add more routes
}
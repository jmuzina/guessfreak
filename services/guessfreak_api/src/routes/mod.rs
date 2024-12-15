use warp::Filter;
use crate::controller::error::error_handler::handle_rejection;

/**
 * All API routes
*/
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    solution::routes()
        .or(files::routes())
        .recover(handle_rejection)
}


mod solution;
mod files;
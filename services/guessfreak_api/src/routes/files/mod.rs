use warp::Filter;
use crate::controller::files;
use crate::model::path::PathRequest;

/**
 * Files routes
*/
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("files")
        .and(
            stream_file()
        )
}

/**
 * Stream a file
 * GET /files/stream_file?path=encoded/path/here
*/
fn stream_file() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("stream_file")
        .and(warp::get())
        .and(warp::query::<PathRequest>())
        .and_then(files::stream_file)
}

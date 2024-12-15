use warp::Filter;
use crate::controller::static_asset;

/**
 * Files routes
*/
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("static_asset")
        .and(
            stream_file()
        )
}

/**
 * Stream a file
 * GET /static_asset/<static_asset_id>
*/
fn stream_file() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path::param::<u64>())
        .and_then(static_asset::stream_static_asset)
}

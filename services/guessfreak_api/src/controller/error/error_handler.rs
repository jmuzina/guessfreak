use warp::{http::StatusCode, Rejection, Reply};

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        Ok(warp::reply::with_status("Not Found", StatusCode::NOT_FOUND))
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        log::error!("Failed to deserialize request body: {:?}", e);
        Ok(warp::reply::with_status("Bad Request", StatusCode::BAD_REQUEST))
    } else if let Some(e) = err.find::<warp::reject::InvalidQuery>() {
        log::error!("Invalid query parameters: {:?}", e);
        Ok(warp::reply::with_status("Invalid query", StatusCode::BAD_REQUEST))
    } else if let Some(e) = err.find::<warp::reject::MethodNotAllowed>() {
        log::error!("Method not allowed: {:?}", e);
        Ok(warp::reply::with_status("Method Not Allowed", StatusCode::METHOD_NOT_ALLOWED))
    } else if let Some(e) = err.find::<warp::reject::MissingCookie>() {
        log::error!("Missing cookie: {:?}", e);
        Ok(warp::reply::with_status("Missing Cookie", StatusCode::BAD_REQUEST))
    } else if let Some(e) = err.find::<warp::reject::MissingHeader>() {
        log::error!("Missing header: {:?}", e);
        Ok(warp::reply::with_status("Missing Header", StatusCode::BAD_REQUEST))
    } else if let Some(e) = err.find::<warp::reject::PayloadTooLarge>() {
        log::error!("Payload too large: {:?}", e);
        Ok(warp::reply::with_status("Payload Too Large", StatusCode::PAYLOAD_TOO_LARGE))
        // unsupportedmediatype
    } else if let Some(e) = err.find::<warp::reject::UnsupportedMediaType>() {
        log::error!("Unsupported media type: {:?}", e);
        Ok(warp::reply::with_status("Unsupported Media Type", StatusCode::UNSUPPORTED_MEDIA_TYPE))
    } else {
        log::error!("Unhandled internal error: {:?}", err);
        Ok(warp::reply::with_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR))
    }
}

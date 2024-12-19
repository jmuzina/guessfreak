use bytes::Bytes;
use crate::db::supabase::storage;
use crate::model::path::PathRequest;
use warp::http::Response;

pub async fn cnt_stream_file(path: PathRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let file_bytes = storage::download_file(path.path.as_ref()).await;

    match file_bytes {
        Ok(contents) => {
            let stream = tokio_stream::iter(vec![Ok::<Bytes, std::io::Error>(Bytes::from(contents))]);

            let response = Response::builder()
                .header("Content-Type", mime_guess::from_path(&path.path).first_or_octet_stream().to_string())
                .header("Content-Disposition", format!("filename=\"{}\"", &path.path.split("/").last().unwrap_or(&path.path)))
                .body(warp::hyper::Body::wrap_stream(stream));

            match response {
                Ok(response) => Ok(response),
                Err(err) => {
                    log::error!("Failed to create response: {}", err);
                    Err(warp::reject::not_found())
                }
            }
        },
        Err(err) => {
            log::error!("Failed to get file: {}", err);
            Err(warp::reject::not_found())
        }
    }
}

use bytes::Bytes;
use warp::http::Response;
use crate::db::supabase::storage::download_file;
use crate::service::static_asset::get_static_asset;

pub async fn cnt_stream_static_asset(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    match get_static_asset(id).await {
        Ok(static_asset) => {
            match download_file(&static_asset.path).await {
                Ok(file_bytes) => {
                    let stream = tokio_stream::iter(vec![Ok::<Bytes, std::io::Error>(Bytes::from(file_bytes))]);

                    let response = Response::builder()
                        .header("Content-Type", mime_guess::from_path(&static_asset.path).first_or_octet_stream().to_string())
                        .header("Content-Disposition", format!("filename=\"{}\"", &static_asset.path.split("/").last().unwrap_or(&static_asset.path)))
                        .body(warp::hyper::Body::wrap_stream(stream));

                    match response {
                        Ok(response)  => Ok(response),
                        Err(err) => {
                            log::error!("Error occurred constructing response body: {}", err);
                            Err(warp::reject::reject())
                        }
                    }
                },
                Err(err) => {
                    log::error!("Error occurred while downloading file: {}", err);
                    Err(warp::reject::reject())
                }
            }
        },
        Err(err) => {
            log::error!("Failed to get file with id {} due to error {}", id, err);
            Err(warp::reject::reject())
        }
    }
}

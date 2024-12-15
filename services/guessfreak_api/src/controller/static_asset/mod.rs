use bytes::Bytes;
use warp::http::Response;
use crate::db::supabase::storage::download_file;
use crate::model::static_asset::StaticAsset;
use crate::service::static_asset::get_static_asset;

pub async fn stream_static_asset(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let static_asset: Option<StaticAsset> = get_static_asset(id).await;

    match static_asset {
        Some(static_asset) => {
            let file_bytes = download_file(&static_asset.path).await;

            match file_bytes {
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
                            Err(warp::reject::not_found())
                        }
                    }
                },
                Err(err) => {
                    log::error!("Error occurred while downloading file: {}", err);
                    Err(warp::reject::not_found())
                }
            }
        },
        None => {
            log::error!("Failed to get file: {}", id);
            Err(warp::reject::not_found())
        }
    }
}

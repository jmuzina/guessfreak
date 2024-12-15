pub mod files {
    use bytes::Bytes;
    use supabase_storage_rs::errors::Error;
    use supabase_storage_rs::models::{FileObject, FileSearchOptions};
    use crate::db::supabase::storage;
    use crate::model::path::PathRequest;
    use tokio_stream::StreamExt;
    use warp::http::Response;

    pub async fn get_files_by_path(path: PathRequest) -> Result<impl warp::Reply, warp::Rejection> {
        let files = storage::get_files(path.path.as_ref(), Option::from(FileSearchOptions::default())).await;
        match files {
            Ok(files) => Ok(warp::reply::json(&files)),
            Err(e) => {
                log::error!("Unexpected error getting files: {:?}", e);
                Err(warp::reject::reject())
            }
        }
    }

    pub async fn get_file(path: PathRequest) -> Result<impl warp::Reply, warp::Rejection> {
        log::info!("File path: {:?}", path.path);
        let file_bytes = storage::get_file(path.path.as_ref()).await;

        match file_bytes {
            Ok(contents) => {
                let stream = tokio_stream::iter(vec![Ok::<Bytes, std::io::Error>(Bytes::from(contents))]);

                let response = Response::builder()
                    .header("Content-Type", mime_guess::from_path(path.path).first_or_octet_stream().to_string())
                    .body(warp::hyper::Body::wrap_stream(stream))
                    .unwrap();

                Ok(response)
            },
            Err(err) => {
                log::error!("Failed to get file: {}", err);
                Err(warp::reject::not_found())
            }
        }
    }

}
pub mod files {
    use supabase_storage_rs::errors::Error;
    use supabase_storage_rs::models::{FileObject, FileSearchOptions};
    use crate::db::supabase::storage;
    use crate::model::path::PathRequest;

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
}
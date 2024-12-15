pub mod files {
    use supabase_storage_rs::errors::Error;
    use supabase_storage_rs::models::{FileObject, FileSearchOptions};
    use crate::db::supabase::storage;

    pub async fn get_files_by_path(path: String) -> Result<impl warp::Reply, warp::Rejection> {
        let files = storage::get_files(path.as_str(), Option::from(FileSearchOptions::default())).await;
        match files {
            Ok(files) => Ok(warp::reply::json(&files)),
            Err(e) => {
                log::error!("Unexpected error getting files: {:?}", e);
                Err(warp::reject::reject())
            }
        }
    }
}
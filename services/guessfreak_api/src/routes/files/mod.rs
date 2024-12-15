pub mod files {
    use warp::Filter;
    use crate::controller::files::files;
    use crate::model::path::PathRequest;

    /**
     * Files routes
    */
    pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("files")
            .and(
                get_files_by_path()
            )
    }

    fn get_files_by_path() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path("get_files")
            .and(warp::get())
            .and(warp::query::<PathRequest>()) // Extract `path` from query string ?path=encoded/path/here
            .and_then(files::get_files_by_path)
    }
}
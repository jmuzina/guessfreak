pub mod files {
    use warp::Filter;
    use crate::controller::files::files;

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
        warp::path::param::<String>()
            .and(warp::get())
            .and_then(files::get_files_by_path)
    }
}
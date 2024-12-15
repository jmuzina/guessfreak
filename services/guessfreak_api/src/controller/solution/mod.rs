pub mod solution {
    use serde_json;
    use crate::db::supabase;
    use crate::model::solution::Solution;

    /**
     * @param id The id of the solution
     * @return The solution
    */
    pub async fn get_solution_by_id(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
        let data = supabase::get_db_client()
            .select("solution")
            .eq("id", id.to_string().as_str())
            .execute()
            .await
            .map_err(|_| warp::reject::not_found())?;

        if data.is_empty() {
            return Err(warp::reject::not_found());
        }

        let solution: Solution = serde_json::from_value(data[0].clone()).map_err(|err| {
            log::error!("Unexpected deserialization error mapping to Solution model for id {}: {:?}", id, err);
            warp::reject::not_found()
        })?;

        Ok(warp::reply::json(&solution))
    }
}
use super::super::model::solution::Solution;
use super::super::db::supabase;
use serde_json;
use log::error;

/**
    * @param id The id of the solution
    * @return The solution
*/
pub async fn get_solution(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let data = supabase::get_db()
        .select("solution")
        .eq("id", id.to_string().as_str())
        .execute()
        .await
        .map_err(|_| warp::reject::not_found())?;

    if data.is_empty() {
        return Err(warp::reject::not_found());
    }

    let solution: Solution = serde_json::from_value(data[0].clone()).map_err(|err| {
        error!("Unexpected deserialization error mapping to Solution model for id {}: {:?}", id, err);
        warp::reject::not_found()
    })?;

    Ok(warp::reply::json(&solution))
}
use dotenv::dotenv;
use env_logger;

mod db;
mod model;
mod controller;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let routes = routes::routes::routes();

    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

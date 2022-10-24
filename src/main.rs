pub mod db;
mod handlers;
mod models;
mod routes;
mod constants;

use mongodb::{bson::{doc, Document},  options::ClientOptions, Client};


#[tokio::main]
async fn main() {
    let mut db = db::Database::init_db("testdb").await;

    let customers_collection = db.create_collection::<models::Customer>("customers").await.expect("failed, maybe due to network");

    let customer_routes = routes::customer_routes(customers_collection);

    warp::serve(customer_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;

    println!("Hello, world!");
}

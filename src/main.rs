mod db;
mod handlers;
mod models;
mod routes;
mod constants;

use mongodb::{bson::{doc, Document},  options::ClientOptions, Client};


#[tokio::main]
async fn main() {
    let mut db = db::Database::<models::Customer>::init_db("testDB").await;

    let customer_routes = routes::customer_routes(db.collection);

    warp::serve(customer_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;

    println!("Hello, world!");
}

// #[tokio::main]
// async fn main() -> mongodb::error::Result<()> {
//     let client_options = ClientOptions::parse(
//             "mongodb+srv://morka:oso32mMgWsP1kpkQ@cluster0.icaw12w.mongodb.net/?retryWrites=true&w=majority",
//         )
//         .await?;
//     let client = Client::with_options(client_options)?;
//     let database = client.database("testdb");

//     let collection = database.collection::<Document>("books");

//     // List the names of the collections in that database.
//     for collection_name in database.list_collection_names(None).await? {
//         println!("{}", collection_name);
//     }


//     Ok(())
// }

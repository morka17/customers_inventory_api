use std::convert::Infallible;
use warp::{self, Filter};


use crate::db::Db;
use crate::handlers;
use crate::models::Customer;



/// Helper function that pass a reference of the data stoer into the 
/// handlers from the routes.
/// 
/// This allows the data store to be injected into the route and passed 
/// along into the handler.

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error= Infallible> + Clone{
    warp::any().map(move || db.clone())
}


/// GET /customers
fn customers_list(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("customers")
    .and(warp::get())
    .and(with_db(db))
    .and_then(handlers::list_customers)
}


pub fn customer_routes(db: Db)
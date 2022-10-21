use std::convert::Infallible;
use warp::{self, Filter};


use crate::db::Db;
use crate::handlers;
use crate::models::Customer;




/// All customer routes
pub fn customer_routes(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_customer(db.clone())
        .or(update_customer(db.clone()))
        .or(delete_customer(db.clone()))
        .or(create_customer(db.clone()))
        .or(customers_list(db))
}


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

fn json_body() -> impl Filter<Extract = (Customer,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 10)
    .and(warp::body::json())
}


/// POST /customers
fn create_customer(db: Db) 
-> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone  {
    warp::path("customers")
    .and(warp::post())
    .and(json_body())
    .and(with_db(db))
    .and_then(handlers::create_customer)
}




// GET /customers/{guid}
fn get_customer(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("customers"/ String)
    .and(warp::get())
    .and(with_db(db))
    .and_then(handlers::get_customer)
}


/// PUT /customers/{guid}
fn update_customer(
db: Db
)-> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    warp::path!("customers" / String)
    .and(warp::put())
    .and(json_body())
    .and(with_db(db))
    .and_then(handlers::update_customer)
}


/// DELETE /customers/{guid}
fn delete_customer(
    db: Db
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("customers" / String)
    .and(warp::delete())
    .and(with_db(db))
    .and_then(handlers::delete_customer)
}
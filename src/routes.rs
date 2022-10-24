use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::collection;
use crate::handlers;
use crate::models::Customer;

/// All customer routes
pub fn customer_routes(
    db: collection<Customer>,
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

fn with_db(
    db: collection<Customer>,
) -> impl Filter<Extract = (collection<Customer>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

/// GET /customers
fn customers_list(
    db: collection<Customer>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("customers")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_customers)
}

fn json_body() -> impl Filter<Extract = (Customer,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 10).and(warp::body::json())
}

/// POST /customers
fn create_customer(
    db: collection<Customer>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("customers")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_customer)
}

// GET /customers/{guid}
fn get_customer(
    db: collection<Customer>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("customers" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_customer)
}

/// PUT /customers/{guid}
fn update_customer(
    db: collection<Customer>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("customers" / String)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::update_customer)
}

/// DELETE /customers/{guid}
fn delete_customer(
    db: collection<Customer>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("customers" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_customer)
}

#[cfg(test)]
mod test {
    use super::*;

    // use warp::Filter;

    fn sum() -> impl Filter<Extract = (u32,), Error = warp::Rejection> + Copy {
        warp::path::param()
            .and(warp::path::param())
            .map(|x: u32, y: u32| x + y)
    }

    fn math() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
        warp::post().and(sum()).map(|z: u32| format!("Sum = {}", z))
    }

    #[tokio::test]
    async fn test_sum() {
        let filter = sum();

        // Execute `sum` and get the `Extract` back.
        let value = warp::test::request()
            .path("/1/2")
            .filter(&filter)
            .await
            .unwrap();

        assert_eq!(value, 3);

        // Or simply test if a request matches (doesn't reject).
        assert!(warp::test::request().path("/1/-5").matches(&filter).await);
    }

    #[tokio::test]
    async fn test_math() {

        let mut db = crate::db::Database::init_db("testdb").await;

        let customers_collection = db
            .create_collection::<crate::models::Customer>("customers")
            .await
            .expect("failed, maybe due to network");

        let res = warp::test::request()
        .reply(&customers_list(customers_collection)).await;

        assert_eq!(res.status(), 200, "should return 200 OK.");
        println!("{:#?}", res.body())
    }
}

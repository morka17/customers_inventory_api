use std::convert::Infallible;
use warp::{self, http::StatusCode};

//use futures::stream::TryStreamExt;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::doc,
    options::{FindOptions, UpdateModifications},
};

use crate::db;
use crate::models::Customer;

/// List Customers
///
///
/// Returns a list of customers as JSON
///
/// # Arguments
///
/// * `db` - `Db` -> thread safe vector of Customer objects
pub async fn list_customers(
    db_collection: db::collection<Customer>,
) -> Result<impl warp::Reply, Infallible> {
    let collections = db_collection.lock().await;
    let collections = collections.clone();

    let filter = doc! {};
    let find_options = FindOptions::builder().sort(doc! {}).build();

    let cursor = collections
        .find(filter, find_options)
        .await
        .expect("no match");

    let cursor: Vec<Customer> = cursor.try_collect().await.expect("could not parsed");

    Ok(warp::reply::json(&cursor))
}

/// Create Customer
///
/// Add a new customer object to the data store if the customer
/// does'nt already exist
///
/// # Arguments
///
/// * `new_customer` - `Customer` type
/// & `db` - `Db` -> thread safe vector of Customer objects
pub async fn create_customer(
    new_customer: Customer,
    db_collection: db::collection<Customer>,
) -> Result<impl warp::Reply, Infallible> {
    let customers = db_collection.lock().await;

    let customers = customers.clone();

    let guid = std::rc::Rc::new(new_customer.guid);

    let filter = doc! {"guid":  guid};
    let _find_options = FindOptions::builder().sort(doc! {}).build();

    match customers.find_one(filter, None).await {
        Ok(_c) => Ok(StatusCode::BAD_REQUEST),
        Err(_) => match customers.insert_one(new_customer, None).await {
            Ok(_) => Ok(StatusCode::CREATED),
            Err(_) => Ok(StatusCode::BAD_REQUEST),
        },
    }
}

/// Get Customer
///
/// Returns a JSON object of an existing customer. if the customer
/// is not found, it returns a NOT FOUND status code.
///
/// # Arguments
///
/// * `guid` - String -> the id of the customer to retrieve
/// * `db` - `Db` -> the thread safe data store
pub async fn get_customer(
    guid: String,
    db_collection: db::collection<Customer>,
) -> Result<Box<dyn warp::Reply>, Infallible> {
    let customers = db_collection.lock().await;

    let customers = customers.clone();

    let filter = doc! {"guid": guid};

    let customer = customers
        .find_one(filter, None)
        .await
        .expect("Network error");

    match customer {
        Some(c) => Ok(Box::new(warp::reply::json(&c))),
        None => Ok(Box::new(StatusCode::NOT_FOUND)),
    }

    // for customer in customers.iter() {
    //     if customer.guid == guid {
    //         return Ok(Box::new(warp::reply::json(&customer)));
    //     }
    // }
    // Ok(Box::new(StatusCode::NOT_FOUND))
}

/// Updates Customer
///
/// Overwrites an existing customer in hte data store and returns
/// an OK status code. if the customer is not found, a Not_FOUND status
/// code is returned.
///
/// # Arguments
///
/// * `update_customer` - `Customer` -> update customer info
/// * `db` - `Db` -> thread safe data store
pub async fn update_customer(
    guid: String,
    updated_customer: Customer,
    db_collection: db::collection<Customer>,
) -> Result<impl warp::Reply, Infallible> {
    let customers = db_collection.lock().await;

    let customers = customers.clone();

    let filter = doc! {"guid": guid};

    let updated = doc! {
        "first_name": updated_customer.first_name,
        "last_name": updated_customer.last_name,
        "email": updated_customer.email,
        "address": updated_customer.address
    };

    let result = customers.update_one(filter, updated, None).await;

    match result {
        Ok(_res) => Ok(StatusCode::OK),
        Err(_) => Ok(StatusCode::NOT_FOUND),
    }
}

/// Deletes Customer
///
/// If the customer exists in the data store, the customer is
/// removed and a NO CONTENT status code is returned.
/// If the customer doesn't exist, a NOT_FOUND status code is returned  
///
/// # Arguments
///
/// * `guid` - String -> the id of the customer to delete
/// * `db` - `Db` -> thread safe data store
pub async fn delete_customer(
    guid: String,
    db_collection: db::collection<Customer>,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db_collection.lock().await;

    let customers = customers.clone();

    let filter = doc! {"guid": guid};

    let result = customers.delete_one(filter, None).await;

    match result {
        Ok(_r) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Ok(StatusCode::NOT_FOUND),
    }
}

use std::sync::Arc;
use tokio::sync::Mutex;
use std::fs::File;
use serde_json::from_reader;

use crate::models::Customer;

pub type Db = Arc<Mutex<Vec<Customer>>>;


pub fn init_db() -> Db {
    let file = File::open("./data/customers.json");
    match file {
        Ok(json) => {
            let customers = from_reader(json).unwrap();
            Arc::new(Mutex::new(customers))
        },
        Err(_) => {
             match File::create("./data/customers.json"){
                Ok(f) => {
                   match  File::open("./data/customers.json"){
                    Ok(json) => {
                        let customers = from_reader(json).unwrap();
                        Arc::new(Mutex::new(customers))
                    },
                    Err(err) => println!("Error opening database {}", err)
                   }
                },
                Err(_) => println!("Error creating database")
            }
            
        }
    }
}
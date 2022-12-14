
use serde::{Deserialize, Serialize};


#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Customer {
    pub guid: String, 
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub address: String,
}


impl crate::db::DbCollections for Customer {
    
}
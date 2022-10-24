use std::sync::Arc;
use tokio::sync::Mutex;

use crate::constants;

use mongodb::{options::ClientOptions, Client};

pub trait DbCollections: Sized {}

pub type collection<T: DbCollections> = Arc<Mutex<mongodb::Collection<T>>>;

#[derive(Debug)]
pub struct Database {
    pub client: mongodb::Client,
    pub db: Arc<Mutex<mongodb::Database>>,
    // pub collection: Arc<Mutex<mongodb::Collection<T>>>,
    //_phantom: std::marker::PhantomData<T>,
}

impl Database {
    pub async fn init_db<'i>(name: &'i str) -> Self {
        let client_options = ClientOptions::parse(format!(
            "mongodb+srv://{}:{}@cluster0.icaw12w.mongodb.net/?retryWrites=true&w=majority",
            constants::username,
            constants::password
        ))
        .await
        .expect("Error connecting to Database");

        let client = Client::with_options(client_options).expect("Couldn't connect to Database");

        let db = client.database(name);
        // let collection = db.collection::<T>("customers");

        Self {
            client: client,
            db: Arc::new(Mutex::new(db)),
            // collection: Arc::new(Mutex::new(collection)),
            // _phantom: std::marker::PhantomData,
        }
    }

    pub async fn create_collection<'i, T>(
        &mut self,
        name: &'i str,
    ) -> Result<Arc<Mutex<mongodb::Collection<T>>>, mongodb::error::Error> {
        self.db.lock().await.create_collection(name, None).await?;

        Ok(Arc::new(Mutex::new(
            self.db.lock().await.collection::<T>(name),
        )))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn database_initialization()  {
     Database::init_db("test1DB").await;
    }

    #[tokio::test]
    async fn create_collection()  {
        let mut db = Database::init_db("test1DB").await;

        let coll = db.create_collection::<crate::models::Customer>("test1_books").await;

        if let Err(err) = coll {
            panic!("{}",err )
        }
    }
}

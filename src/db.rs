use std::sync::Arc;
use tokio::sync::Mutex;

use crate::constants;

use mongodb::{options::ClientOptions, Client};

pub trait DbCollections: Sized {}

pub type collection<T: DbCollections> = Arc<Mutex<mongodb::Collection<T>>>;

#[derive(Debug)]
pub struct Database<T: DbCollections> {
    client: mongodb::Client,
    db: mongodb::Database,
    pub collection: Arc<Mutex<mongodb::Collection<T>>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: DbCollections> Database<T> {
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
        let collection = db.collection::<T>("customers");

        Self {
            client: client,
            db: db,
            collection: Arc::new(Mutex::new(collection)),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get_collection<'i>(&mut self, name: &'i str) -> Arc<Mutex<mongodb::Collection<T>>> {
        Arc::new(Mutex::new(self.db.collection::<T>(name)))
    }
}

// morka qqKi2lrv2mvLQ1Z3
//
// morka oso32mMgWsP1kpkQ

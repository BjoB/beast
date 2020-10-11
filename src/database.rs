use crate::config::*;
use mongodb::bson::doc;
use mongodb::sync::Client;

#[derive(Clone, Debug)]
pub struct DataBase {
    pub client: Client,
    pub dbname: String,
}

impl DataBase {
    pub fn init(config: &AppConfig) -> DataBase {
        let mongodb_uri = config.mongodb_uri();
        let mongodb_name = config.mongodb_name();

        let client = Client::with_uri_str(mongodb_uri)
            .expect(&format!("Invalid database uri: {}.", mongodb_uri));

        println!("Checking database connection ...");

        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .expect("Could not connect to database!");

        println!("Connected successfully!");

        Self {
            client: client,
            dbname: mongodb_name.to_string(),
        }
    }
}

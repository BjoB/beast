use crate::config::*;
use crate::parse::*;

use mongodb::bson;
use mongodb::bson::{Bson, Regex};
use mongodb::sync::{Client, Collection};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct DataBase {
    pub client: Client,
    pub dbname: String,
    pub collection: String,
}

pub enum EntryFilter {
    ExeName(String),
    //Tag(String),
    //All(String, String),
}

impl DataBase {
    pub fn init(config: &AppConfig) -> DataBase {
        let mongodb_uri = config.mongodb_uri();
        let mongodb_name = config.mongodb_name();
        let mongodb_collection = config.mongodb_collection();

        let client = Client::with_uri_str(mongodb_uri)
            .expect(&format!("Invalid database uri: {}.", mongodb_uri));

        println!("Checking database connection ...");

        client
            .database(mongodb_name)
            .run_command(bson::doc! {"ping": 1}, None)
            .expect("Could not connect to database!");

        println!("Connected successfully!");

        Self {
            client: client,
            dbname: mongodb_name.to_string(),
            collection: mongodb_collection.to_string(),
        }
    }

    pub fn push_last_results(&self, tag: Option<String>) {
        let tag_value = tag.unwrap_or("".to_string());
        let benchmark_collection = self.benchmark_collection();

        let cumulated_results = parse_cumulated_benchmark_file();
        let mut docs = vec![];
        for result in cumulated_results {
            let result_bson = bson::to_bson(&result).unwrap();
            let exe_name = exe_name(&result.context.executable);
            docs.push(bson::doc! {"exe_name": Bson::String(exe_name), "tag": Bson::String(tag_value.clone()), "results": result_bson});
        }

        benchmark_collection
            .insert_many(docs, None)
            .expect("Could not insert benchmark results in database collection!");
    }

    pub fn fetch(&self, entry_filter: EntryFilter) -> Vec<DataBaseEntry> {
        let benchmark_collection = self.benchmark_collection();

        let filter = match entry_filter {
            EntryFilter::ExeName(reg_expr) => {
                bson::doc! { "exe_name": Regex{pattern: reg_expr, options: String::new()} }
            }
            //EntryFilter::Tag(tag) => bson::doc! { "tag": tag },
            //EntryFilter::All(exe, tag) => bson::doc! { "exe_name": exe, "tag": tag },
        };
        println!("Using mongodb query: {}", filter);

        let cursor = benchmark_collection
            .find(filter, None)
            .expect("Could not fetch results from database!");

        let mut fetched_results = vec![];

        for result in cursor {
            match result {
                Ok(document) => {
                    let entry: DataBaseEntry = bson::from_bson(Bson::Document(document))
                        .expect("Could not deserialize database entry!");
                    fetched_results.push(entry);
                }
                Err(e) => panic!(e),
            }
        }

        fetched_results
    }

    pub fn list_tags(&self) -> Vec<String> {
        let benchmark_collection = self.benchmark_collection();

        let tags = benchmark_collection
            .distinct(
                "tag",
                bson::doc! {"tag": {"$exists" : true, "$ne" : ""} },
                None,
            )
            .expect("Could not retrieve list of tags!");

        tags.iter()
            .map(|tag| bson::from_bson(tag.clone()).unwrap())
            .collect()
    }

    fn benchmark_collection(&self) -> Collection {
        self.client
            .database(&self.dbname)
            .collection(&self.collection)
    }
}

fn exe_name(pathbuf: &PathBuf) -> String {
    pathbuf
        .as_path()
        .file_name()
        .unwrap()
        .to_str()
        .expect("Could not convert executable path to valid string!")
        .to_string()
}

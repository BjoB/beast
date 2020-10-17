use clap::crate_name;
use preferences::{AppInfo, Preferences, PreferencesMap};

pub const APP_INFO: AppInfo = AppInfo {
    name: crate_name!(),
    author: "beastuser",
};

const DATABASE_PREFS_KEY: &str = "preferences/mongodb";
const DATABASE_URI_KEY: &str = "url";
const DATABASE_NAME_KEY: &str = "database_name";
const DATABASE_BENCHMARK_COLLECTION_KEY: &str = "collection_name";

pub struct AppConfig {
    preferences: PreferencesMap<String>,
}

impl AppConfig {
    pub fn init() -> AppConfig {
        let loaded_config = match PreferencesMap::<String>::load(&APP_INFO, &DATABASE_PREFS_KEY) {
            Ok(cfg) => cfg,
            Err(_) => {
                // Set default config and return it
                let mut default_cfg: PreferencesMap<String> = PreferencesMap::new();
                default_cfg.insert(DATABASE_URI_KEY.into(), "".into());
                default_cfg.insert(DATABASE_NAME_KEY.into(), "".into());
                default_cfg
            }
        };

        AppConfig {
            preferences: loaded_config,
        }
    }

    pub fn print(&self) {
        println!("Currently loaded config: {:?}", self.preferences);
    }

    pub fn is_db_config_set(&self) -> bool {
        return !self.mongodb_uri().is_empty() && !self.mongodb_name().is_empty();
    }

    pub fn set_mongodb_uri(&mut self, url: &String) {
        self.preferences.insert(DATABASE_URI_KEY.into(), url.into());
        self.save();
    }

    pub fn set_mongodb_name(&mut self, name: &String) {
        self.preferences
            .insert(DATABASE_NAME_KEY.into(), name.into());
        self.save();
    }

    pub fn set_mongodb_collection(&mut self, name: &String) {
        self.preferences
            .insert(DATABASE_BENCHMARK_COLLECTION_KEY.into(), name.into());
        self.save();
    }

    pub fn mongodb_uri(&self) -> &String {
        self.preferences
            .get(DATABASE_URI_KEY)
            .expect("Can't retrieve mongodb url from config!")
    }

    pub fn mongodb_name(&self) -> &String {
        self.preferences
            .get(DATABASE_NAME_KEY)
            .expect("Can't retrieve mongodb database name from config!")
    }

    pub fn mongodb_collection(&self) -> &String {
        self.preferences
            .get(DATABASE_BENCHMARK_COLLECTION_KEY)
            .expect("Can't retrieve mongodb collection name from config!")
    }

    fn save(&self) {
        self.preferences
            .save(&APP_INFO, &DATABASE_PREFS_KEY)
            .expect("Failed to save new default config!");
        println!("Config successfully saved: {:?}", self.preferences);
    }
}

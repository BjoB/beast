use clap::crate_name;
use colored::Colorize;
use preferences::{AppInfo, Preferences, PreferencesMap};

pub const APP_INFO: AppInfo = AppInfo {
    name: crate_name!(),
    author: "beastuser",
};

const DATABASE_CONFIG_PATH: &str = "preferences/mongodb";
const DATABASE_URI_KEY: &str = "url";
const DATABASE_NAME_KEY: &str = "database_name";
const DATABASE_BENCHMARK_COLLECTION_KEY: &str = "collection_name";

const GIT_CONFIG_PATH: &str = "preferences/git";
const GIT_YAML_PATH_KEY: &str = "repocheck_yaml_path";

pub struct AppConfig {
    db_config: PreferencesMap<String>,
    git_config: PreferencesMap<String>,
}

impl AppConfig {
    pub fn init() -> AppConfig {
        let loaded_db_config =
            match PreferencesMap::<String>::load(&APP_INFO, &DATABASE_CONFIG_PATH) {
                Ok(cfg) => cfg,
                Err(_) => {
                    // Set default config and return it
                    let mut default_cfg: PreferencesMap<String> = PreferencesMap::new();
                    default_cfg.insert(DATABASE_URI_KEY.into(), "".into());
                    default_cfg.insert(DATABASE_NAME_KEY.into(), "".into());
                    default_cfg
                }
            };

        let loaded_git_config = match PreferencesMap::<String>::load(&APP_INFO, &GIT_CONFIG_PATH) {
            Ok(cfg) => cfg,
            Err(_) => {
                // Set default config and return it
                let mut default_cfg: PreferencesMap<String> = PreferencesMap::new();
                default_cfg.insert(GIT_YAML_PATH_KEY.into(), "".into());
                default_cfg
            }
        };

        AppConfig {
            db_config: loaded_db_config,
            git_config: loaded_git_config,
        }
    }

    // Public helper functions
    pub fn print(&self) {
        println!("{}", "Currently loaded database config:".cyan());
        for (key, value) in &self.db_config {
            println!("{} : \"{}\"", key, value);
        }
        println!("\n{}", "Currently loaded repocheck config:".cyan());
        for (key, value) in &self.git_config {
            println!("{} : \"{}\"", key, value);
        }
    }

    pub fn is_db_config_set(&self) -> bool {
        return !self.mongodb_uri().is_empty() && !self.mongodb_name().is_empty();
    }

    // Config setter
    pub fn set_mongodb_uri(&mut self, url: &String) {
        self.set_db_config_value(DATABASE_URI_KEY, url);
    }

    pub fn set_mongodb_name(&mut self, name: &String) {
        self.set_db_config_value(DATABASE_NAME_KEY, name);
    }

    pub fn set_mongodb_collection(&mut self, name: &String) {
        self.set_db_config_value(DATABASE_BENCHMARK_COLLECTION_KEY, name);
    }

    pub fn set_repocheck_config_yaml(&mut self, repo_url: &String) {
        self.set_git_config_value(GIT_YAML_PATH_KEY, repo_url);
    }

    // Config getter
    pub fn mongodb_uri(&self) -> &String {
        self.get_db_config_value(DATABASE_URI_KEY)
    }

    pub fn mongodb_name(&self) -> &String {
        self.get_db_config_value(DATABASE_NAME_KEY)
    }

    pub fn mongodb_collection(&self) -> &String {
        self.get_db_config_value(DATABASE_BENCHMARK_COLLECTION_KEY)
    }

    pub fn repocheck_config_yaml(&self) -> &String {
        self.get_git_config_value(GIT_YAML_PATH_KEY)
    }

    // Private helper functions
    fn set_db_config_value(&mut self, key: &str, value: &str) {
        self.db_config.insert(key.into(), value.into());
        self.db_config
            .save(&APP_INFO, DATABASE_CONFIG_PATH)
            .expect("Failed to save new default db config!");
        println!("Config successfully saved: {:?}", self.db_config);
    }

    fn get_db_config_value(&self, key: &str) -> &String {
        self.db_config
            .get(key)
            .expect(&format!("Can't retrieve config value for key '{}'!", key))
    }

    fn set_git_config_value(&mut self, key: &str, value: &str) {
        self.git_config.insert(key.into(), value.into());
        self.git_config
            .save(&APP_INFO, GIT_CONFIG_PATH)
            .expect("Failed to save new default git config!");
        println!("Config successfully saved: {:?}", self.git_config);
    }

    fn get_git_config_value(&self, key: &str) -> &String {
        self.git_config
            .get(key)
            .expect(&format!("Can't retrieve config value for key '{}'!", key))
    }
}

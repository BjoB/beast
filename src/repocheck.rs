use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
//use yaml_rust::YamlLoader;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct RepocheckSettings {
    repo_path: PathBuf,
    branch_name: String,
    from_commit: String,
    to_commit: String,
    build_commands: String,
    benchmark_regex: String,
}

pub fn parse_repocheck_settings<P: AsRef<Path>>(yaml_path: P) {
    match File::open(yaml_path) {
        Ok(f) => {
            let yaml_str: String = serde_yaml::from_reader(BufReader::new(f)).unwrap();
            println!("{}", yaml_str); //TODO
        }
        Err(e) => log::error!("Could not open repocheck file ({})", e),
    };
}

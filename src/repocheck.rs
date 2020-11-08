use crate::logger::*;

use git2::Repository;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct RepocheckSettings {
    version: u32,
    repo_path: PathBuf,
    branch_name: String,
    from_commit: String,
    to_commit: String,
    build_commands: String,
    benchmark_regex: String,
}

pub fn parse<P: AsRef<Path>>(yaml_path: P) -> RepocheckSettings {
    match File::open(yaml_path) {
        Ok(f) => match serde_yaml::from_reader(BufReader::new(f)) {
            Ok(yaml_val) => yaml_val,
            Err(e) => error_and_exit("repocheck yaml has invalid format", &e),
        },
        Err(e) => {
            error_and_exit("Could not open repocheck yaml", &e);
        }
    }
}

pub fn run(settings: &RepocheckSettings) {
    let full_repo_path = match std::fs::canonicalize(settings.repo_path.as_path()) {
        Ok(path) => path,
        Err(e) => {
            error_and_exit("Invalid path to repository", &e);
        }
    };

    let _repo = match Repository::open(full_repo_path.as_path()) {
        Ok(repo) => repo,
        Err(e) => {
            error_and_exit("Could not open repository", &e);
        }
    };

    // TODO ...
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let test_yaml_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("doc/example_repocheck_settings.yaml");
        let parsed_settings = parse(test_yaml_path);
        assert_eq!(parsed_settings.version, 1);
        assert_eq!(parsed_settings.repo_path.as_path(), Path::new("."));
        assert_eq!(parsed_settings.branch_name, "master");
    }
}

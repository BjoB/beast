use crate::logger::*;

use colored::*;
use git2::{Error, Oid, Repository};
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

    let repo = match Repository::open(full_repo_path.as_path()) {
        Ok(repo) => repo,
        Err(e) => {
            error_and_exit("Could not open repository", &e);
        }
    };

    // TODO: Also check if repo state is clean

    println!("Checking specified from/to commit range...");
    if let Err(e) = check_commits(&repo, &settings) {
        error_and_exit("Did not find commit hash(es)", &e);
    }
    println!("{}", "Successful!".green());
}

fn check_commits(repo: &Repository, settings: &RepocheckSettings) -> Result<(), Error> {
    let from_commit_oid = Oid::from_str(settings.from_commit.as_str())?;
    let to_commit_oid = Oid::from_str(settings.to_commit.as_str())?;
    let from_commit_parent = repo.find_commit(from_commit_oid)?.parent(0)?;

    let mut revwalk = repo.revwalk()?;
    revwalk.push(to_commit_oid)?;
    revwalk.hide(from_commit_parent.id())?;
    revwalk.simplify_first_parent()?;
    revwalk.set_sorting(git2::Sort::REVERSE | git2::Sort::TIME)?;

    let mut count = 0;
    for id in revwalk {
        count += 1;
        let id = id?;
        println!("{}", id);
    }
    println!("Number of commits found: {}", count);

    Ok(())
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

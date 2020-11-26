use crate::exec::*;
use crate::find::*;
use crate::logger::*;
use crate::parse::*;

use colored::*;
use execute::{shell, Execute};
use git2::{Commit, Error, Oid, Repository};
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::fs::{self, File};
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

pub fn collect_repocheck_results(settings: &RepocheckSettings) -> Vec<BenchmarkResults> {
    let full_repo_path = match std::fs::canonicalize(settings.repo_path.as_path()) {
        Ok(path) => path,
        Err(e) => {
            error_and_exit("Invalid path to repository", &e);
        }
    };

    let export_dir = export_dir(&full_repo_path, &settings.branch_name);

    let mut collected_benchmark_results: Vec<BenchmarkResults> = vec![];
    if export_dir.is_dir() {
        println!("Files to parse:");
        for entry in fs::read_dir(export_dir).unwrap() {
            let repocheck_file_path = entry.unwrap().path();
            println!("{}", &repocheck_file_path.to_string_lossy());
            let single_file_results = json_from_file(repocheck_file_path.as_path());
            let mut json: Vec<BenchmarkResults> = serde_json::from_value(single_file_results)
                .expect("Could not deserialize JsonValue from single benchmark file!");
            collected_benchmark_results.append(&mut json);
        }
    }
    collected_benchmark_results
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

    if repo.state() != git2::RepositoryState::Clean {
        let e = Error::from_str("RepositoryState != Clean");
        error_and_exit("Clean up repository state first!", &e);
    }

    println!(
        "Checking out branch '{}' in repository '{}'...",
        settings.branch_name,
        full_repo_path.to_string_lossy()
    );
    if let Err(e) = checkout_branch(&repo, &settings.branch_name) {
        error_and_exit("Could not checkout specified branch", &e);
    }
    println!("{}\n", "Successful!".green());

    println!("Walking specified commit range...");
    if let Err(e) = walk_commits(&repo, &full_repo_path, &settings) {
        error_and_exit("Could not walk through specified commit range", &e);
    }
    println!("{}\n", "Successful!".green());
}

fn walk_commits(
    repo: &Repository,
    full_repo_path: &Path,
    settings: &RepocheckSettings,
) -> Result<(), Error> {
    let from_commit_oid = Oid::from_str(settings.from_commit.as_str())?;
    let to_commit_oid = Oid::from_str(settings.to_commit.as_str())?;
    let from_commit_parent = repo.find_commit(from_commit_oid)?.parent(0)?;

    let mut revwalk = repo.revwalk()?;
    revwalk.push(to_commit_oid)?;
    revwalk.hide(from_commit_parent.id())?;
    revwalk.simplify_first_parent()?;
    revwalk.set_sorting(git2::Sort::REVERSE | git2::Sort::TIME)?;

    for rev in revwalk {
        let commit = repo.find_commit(rev?)?;
        checkout_commit(repo, &commit)?;

        println!("Building for commit {}...", &commit.id());

        let repo_workdir = repo.workdir().unwrap();
        println!(
            "Using working directory {}...",
            repo_workdir.to_string_lossy()
        );

        for cmd in settings.build_commands.lines() {
            println!("Executing cmd: {}", cmd.blue());
            let mut build_cmd = shell(cmd);
            let output = match build_cmd.current_dir(repo_workdir).execute_output() {
                Ok(res) => res,
                Err(e) => error_and_exit("Command execution error", &e),
            };
            let output_str = String::from_utf8(output.stdout).unwrap();
            print!("{}\n", output_str.as_str());
        }

        println!("{}\n", "Successful!".green());

        let benchmark_paths = find_executables(repo_workdir, &settings.benchmark_regex);
        let mut results = execute_benchmarks(benchmark_paths);

        let commit_id_str = id_to_str(commit.id().as_bytes());
        append_commit_id(&mut results, &commit_id_str);

        let export_file_name = "commit_".to_string() + commit_id_str.as_str() + ".json";
        let export_dir = export_dir(full_repo_path, &settings.branch_name);
        // TODO: delete dir if existent?

        let mut export_file_path = export_dir;
        export_file_path.push(Path::new(&export_file_name));

        // create parent dir
        let export_parent_dir = export_file_path.parent().unwrap();
        std::fs::create_dir_all(export_parent_dir).unwrap();

        export_results_to_file(&results, export_file_path.as_path());
    }

    Ok(())
}

fn checkout_branch(repo: &Repository, branch_name: &str) -> Result<(), Error> {
    let obj = repo.revparse_single(&("refs/heads/".to_owned() + branch_name))?;
    repo.checkout_tree(&obj, None)?;
    repo.set_head(&("refs/heads/".to_owned() + branch_name))?;
    Ok(())
}

fn checkout_commit(repo: &Repository, commit: &Commit) -> Result<(), Error> {
    repo.checkout_tree(commit.as_object(), None)?;
    repo.set_head_detached(commit.id())?;
    Ok(())
}

fn export_dir(repo_path: &Path, branch_name: &str) -> PathBuf {
    let mut export_file_path = preferences::prefs_base_dir().unwrap();
    export_file_path.push(Path::new("beastrepocheck"));
    export_file_path.push(repo_path.components().last().unwrap());
    export_file_path.push(Path::new(branch_name));
    export_file_path
}

fn append_commit_id(results: &mut Vec<BenchmarkResults>, commit_id_str: &str) {
    for result in results {
        result.commit = Some(commit_id_str.to_string());
    }
}

fn id_to_str(oid: &[u8]) -> String {
    let mut oid_str = String::new();
    for &byte in &oid[0..4] {
        write!(&mut oid_str, "{:02x}", byte).expect("Could not write commit ID byte!");
    }
    oid_str
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

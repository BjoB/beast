use is_executable::IsExecutable;
use regex::Regex;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, Error, WalkDir};

pub fn find_executables<P: AsRef<Path>>(root: P, exe_pattern: &str) -> Vec<PathBuf> {
    let re = Regex::new(exe_pattern).expect(&format!("Could not compile regex: {}", exe_pattern));

    WalkDir::new(root)
        .into_iter()
        .filter_map(|entry: Result<DirEntry, Error>| entry.ok())
        .filter(|entry| entry.file_type().is_file() && entry.path().is_executable())
        .filter(|entry| {
            re.is_match(
                entry
                    .path()
                    .file_name()
                    .expect("This should not happen!")
                    .to_str()
                    .expect("Conversion of filename to string failed!"),
            )
        })
        .map(|entry| entry.path().to_owned())
        .collect()
}

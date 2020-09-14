use is_executable::IsExecutable;
use glob::Pattern;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, Error, WalkDir};

pub fn find_executable_benchmarks<P: AsRef<Path>>(root: P) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|entry: Result<DirEntry, Error>| entry.ok())
        .filter(|entry| entry.file_type().is_file() && entry.path().is_executable())
        .filter(|entry| {
            Pattern::new("*-benchmark*")
                .unwrap()
                .matches(entry.path().to_str().expect("Should not happen!"))
        })
        .map(|entry| entry.path().to_owned())
        .collect()
}

// use std::path::Path;
use std::fs::metadata;
use glob::{glob_with, MatchOptions};
use is_executable::IsExecutable;

fn main() -> Result<(), std::io::Error> {
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    for entry in glob_with("/home/bat/workspace/beast/**/*benchmark*", options).unwrap() {
        if let Ok(path) = entry {
            let md = metadata(&path).unwrap();
            if path.as_path().is_executable() && !md.is_dir() {
                println!("{:?}", path.display())
            }
        }
    }

    Ok(())
}

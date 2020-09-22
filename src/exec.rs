use std::path::PathBuf;
use std::process::Command;

pub fn execute<PathList: AsRef<Vec<PathBuf>>>(exe_paths: PathList) {
    for exe_path in exe_paths.as_ref() {
        let exe_output = Command::new(exe_path)
            .output()
            .expect("failed to execute process");
        // println!("{:?}", exe_output);
    }
}

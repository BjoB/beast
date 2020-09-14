use std::path::Path;

mod findbench;

use findbench::find_executable_benchmarks;

fn main() -> Result<(), std::io::Error> {
    let cwd = Path::new("/home/bat/workspace").to_path_buf();
    let benchmark_paths = find_executable_benchmarks(cwd);
    println!("{:?}", benchmark_paths);
    Ok(())
}

use clap::{App, Arg};
use std::path::Path;

mod findbench;

use findbench::find_executable_benchmarks;

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("beast")
        .version("0.1")
        .about("(be)nchmark (a)nalysis and (s)ummary (t)ool")
        .arg(Arg::from_usage(
            "[rootdir], -d, --dir=[DIR] 'root directory to use for benchmark search'",
        ))
        .get_matches();

    let root_dir = match matches.value_of("rootdir") {
        Some(valid_val) => Path::new(valid_val).to_path_buf(),
        _ => match std::env::current_dir() {
            Ok(path_buf) => path_buf,
            Err(err) => panic!("Can't retrieve current directory: {:?}", err),
        },
    };

    println!("Root scan directory: {:?}", root_dir.as_os_str());

    let benchmark_paths = find_executable_benchmarks(root_dir);
    println!("{:?}", benchmark_paths);
    Ok(())
}

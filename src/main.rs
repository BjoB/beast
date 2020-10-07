use clap::{crate_version, App, Arg};
use std::path::Path;

mod exec;
mod find;
mod parse;
mod plot;

use exec::execute_benchmarks;
use find::find_executables;

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("beast")
        .version(crate_version!())
        .about("(be)nchmark (a)nalysis and (s)ummary (t)ool")
        .arg(Arg::from_usage(
            "[rootdir], -d, --dir=[DIR] 'root directory to use for benchmark search'",
        ))
        .arg(
            Arg::from_usage(
                "[filter], -f, --filter=[REGEXP] 'only run benchmarks matching regular expression'",
            )
            .default_value("*benchmark*"),
        )
        .arg(
            Arg::from_usage(
                "[timeunit], -t, --timeunit=[TIMEUNIT] 'time unit for plots (possible values are: ms, us, ns)'",
            )
            .default_value("us"),
        )
        .get_matches();

    let root_dir = match matches.value_of("rootdir") {
        Some(valid_val) => Path::new(valid_val).to_path_buf(),
        _ => match std::env::current_dir() {
            Ok(path_buf) => path_buf,
            Err(err) => panic!("Can't retrieve current directory: {:?}", err),
        },
    };

    let filter_pattern = matches.value_of("filter").unwrap();
    let plot_time_unit = matches.value_of("timeunit").unwrap();

    println!("Root scan directory: {:?}", root_dir.as_os_str());

    let benchmark_paths = find_executables(root_dir, filter_pattern);

    if benchmark_paths.is_empty() {
        println!("No benchmarks found to run!");
        return Ok(());
    }

    execute_benchmarks(benchmark_paths, plot_time_unit);
    Ok(())
}

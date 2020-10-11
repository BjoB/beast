use clap::{crate_name, crate_version, App, Arg, SubCommand};
use std::path::Path;

mod config;
mod database;
mod exec;
mod find;
mod parse;
mod plot;

use config::*;
use database::*;
use exec::execute_benchmarks;
use find::find_executables;

fn main() -> Result<(), std::io::Error> {
    let matches = App::new(crate_name!())
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
        .subcommand(SubCommand::with_name("config")
            .about("Handle config for e.g. mongodb access")
            .arg(
                Arg::from_usage(
                    "[mongodb_uri], --set-db-url=[URL] 'set a mongodb url for benchmark result collection'",
                )   
            )
            .arg(
                Arg::from_usage(
                    "[mongodb_dbname], --set-db-name=[NAME] 'set a mongodb database to push benchmark results to'",
                )   
            )
        )
        .get_matches();

    let mut config = AppConfig::init();

    // configuration handling
    if let Some(ref matches) = matches.subcommand_matches("config") {
        config.print();
        match matches.value_of("mongodb_uri") {
            Some(provided_url) => config.set_mongodb_uri(&provided_url.to_string()),
            None => {},
        }
        match matches.value_of("mongodb_dbname") {
            Some(provided_mongodb_name) => config.set_mongodb_name(&provided_mongodb_name.to_string()),
            None => {},
        }
        return Ok(());
    }
    config.print();

    // main program handling
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

    if config.is_db_config_set() {
        let _db = DataBase::init(&config);
        // TODO: continue with database handling, e.g. only do this when flag is provided
    } else {
        println!("database config is not yet set. Use 'beast config' for this.");
    }

    return Ok(());
}

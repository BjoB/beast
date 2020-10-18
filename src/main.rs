use clap::{crate_name, crate_version, App, Arg, ArgMatches, SubCommand};
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
use plot::*;

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
                    "[mongodb_uri], --set-db-url=[URL] 'set a mongodb url to push/fetch benchmark results to/from'",
                ),
            )
            .arg(
                Arg::from_usage(
                    "[mongodb_dbname], --set-db-name=[NAME] 'set a mongodb database to access'",
                ),
            )
            .arg(
                Arg::from_usage(
                    "[mongodb_collection], --set-db-collection=[COLLECTION] 'set a mongodb collection to push benchmark results to'",
                ),
            )
        )
        .subcommand(SubCommand::with_name("plotlast")
            .about("Plot benchmark results from last run")
        )
        .subcommand(SubCommand::with_name("dbpush")
            .about("Push previously exported benchmark results to the configured database")
            .arg(
                Arg::from_usage(
                    "[tag], --tag=[TAGNAME] 'add a tag to the pushed results'",
                ),
            )
        )
        .subcommand(SubCommand::with_name("dbplot")
            .about("Fetches all benchmark results from the configured database and plot them as time series")
            .arg(
                Arg::from_usage(
                    "[fetchfilter], --fetchfilter=[REGEXP] 'filter executables to plot with a mongodb regex'",
                )
                .default_value(".*"),
            )
        )
        .get_matches();

    let mut config = AppConfig::init();

    // Handle subcommands
    handle_config_commands(&matches, &mut config);
    handle_database_commands(&matches, &config);

    // Parse main options
    let root_dir = match matches.value_of("rootdir") {
        Some(valid_val) => Path::new(valid_val).to_path_buf(),
        _ => match std::env::current_dir() {
            Ok(path_buf) => path_buf,
            Err(err) => panic!("Can't retrieve current directory: {:?}", err),
        },
    };

    let filter_pattern = matches.value_of("filter").unwrap();
    let plot_time_unit = matches.value_of("timeunit").unwrap();

    // Plot last resuls
    if let Some(ref _matches) = matches.subcommand_matches("plotlast") {
        let last_results = parse::parse_cumulated_benchmark_file();
        plot_all(&last_results, plot_time_unit);
        return Ok(());
    }

    // Benchmark execution handling
    println!("Root scan directory: {:?}", root_dir.as_os_str());

    let benchmark_paths = find_executables(root_dir, filter_pattern);

    if benchmark_paths.is_empty() {
        println!("No benchmarks found to run!");
        return Ok(());
    }

    execute_benchmarks(benchmark_paths, plot_time_unit);

    return Ok(());
}

fn handle_config_commands(matches: &ArgMatches, config: &mut AppConfig) {
    if let Some(ref matches) = matches.subcommand_matches("config") {
        config.print();
        match matches.value_of("mongodb_uri") {
            Some(provided_url) => config.set_mongodb_uri(&provided_url.to_string()),
            None => {}
        }
        match matches.value_of("mongodb_dbname") {
            Some(provided_mongodb_name) => {
                config.set_mongodb_name(&provided_mongodb_name.to_string())
            }
            None => {}
        }
        match matches.value_of("mongodb_collection") {
            Some(provided_mongodb_collection) => {
                config.set_mongodb_collection(&provided_mongodb_collection.to_string())
            }
            None => {}
        }
        std::process::exit(0);
    }
}

fn handle_database_commands(matches: &ArgMatches, config: &AppConfig) {
    let plot_time_unit = matches.value_of("timeunit").unwrap();

    if let Some(ref _matches) = matches.subcommand_matches("dbpush") {
        if config.is_db_config_set() {
            let db = DataBase::init(&config);
            db.push_last_results(None);
        } else {
            println!("database config is not yet set. Use 'beast config' for this.");
        }
        std::process::exit(0);
    }
    if let Some(ref _matches) = matches.subcommand_matches("dbplot") {
        if config.is_db_config_set() {
            let filter_pattern = matches.value_of("fetchfilter").unwrap_or(".*");
            let db = DataBase::init(&config);

            let results = db.fetch(EntryFilter::ExeName(filter_pattern.to_string()));
            //TODO: support "tag" + "both"

            if results.is_empty() {
                println!("Did not find any matching results. Nothing to plot!");
                std::process::exit(0);
            }

            plot_db_entries(&results, plot_time_unit);
        } else {
            println!("database config is not yet set. Use 'beast config' for this.");
        }
        std::process::exit(0);
    }
}

use clap::{crate_name, crate_version, App, Arg, ArgMatches, SubCommand};
use std::path::Path;

mod config;
mod database;
mod exec;
mod find;
mod parse;
mod plot;
mod repocheck;

use config::*;
use database::*;
use exec::execute_benchmarks;
use find::find_executables;
use plot::*;
use repocheck::*;
use simple_logger::SimpleLogger;

fn main() -> Result<(), std::io::Error> {
    SimpleLogger::new().init().unwrap();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about("(be)nchmark (a)nalysis and (s)ummary (t)ool")
        .arg(Arg::from_usage(
            "[rootdir], -d, --dir=[DIR] 'Root directory to use for benchmark search'",
        ))
        .arg(
            Arg::from_usage(
                "[filter], -f, --filter=[REGEXP] 'Only run benchmark executables matching the regex pattern'",
            )
            .default_value(".*benchmark[^.]*$"),
        )
        .arg(
            Arg::with_name("list")
                .help("List benchmark executables found by the current regex (see '-f')")
                .short("l")
                .long("list")
                //.requires("filter")
        )
        .arg(
            Arg::from_usage(
                "[timeunit], -t, --timeunit=[TIMEUNIT] 'Time unit for plots (possible values are: ms, us, ns)'",
            )
            .default_value("us"),
        )
        .arg(
            Arg::with_name("noplot")
                .help("Do not create plot for benchmark results, e.g. when using beast in scripts")
                .long("noplot")
        )
        .subcommand(SubCommand::with_name("config")
            .about("Handle beast's configuration, e.g. the mongodb access or the git settings")
            .arg(
                Arg::from_usage(
                    "[mongodb_uri], --set-db-uri=[URI] 'Sets a mongodb URI for push/fetch of benchmark results'",
                ),
            )
            .arg(
                Arg::from_usage(
                    "[mongodb_dbname], --set-db-name=[NAME] 'Sets a mongodb database to work with'",
                ),
            )
            .arg(
                Arg::from_usage(
                    "[mongodb_collection], --set-db-collection=[COLLECTION] 'Sets a mongodb collection to work with'",
                ),
            )
            .arg(
                Arg::from_usage(
                    "[repocheck_yaml_path], --set-repocheck-yaml=[PATH] 'Sets path to the repocheck settings yaml file'",
                ),
            )
        )
        .subcommand(SubCommand::with_name("plotlast")
            .about("Plots benchmark results from last run \n\
                    Note: Supports the '-t' option after main command to plot with desired time unit.")
        )
        .subcommand(SubCommand::with_name("dbpush")
            .about("Pushes previously exported benchmark results to the configured database")
            .arg(
                Arg::from_usage(
                    "[tag], --tag=[TAGNAME] 'Adds a tag to the pushed results.'",
                ),
            )
        )
        .subcommand(SubCommand::with_name("dbplot")
            .about("Fetches all benchmark results from the configured database collection and plot them as time series \n\
                    Note: Supports the '-t' option after main command to plot with desired time unit.")
            .arg(
                Arg::from_usage(
                    "[fetchfilter], --fetchfilter=[REGEXP] 'Filters executables to plot with a mongodb compatible regexp'",
                )
                .default_value(".*"),
            )
        )
        .subcommand(SubCommand::with_name("dblist")
            .about("Lists distinct tags in current benchmark collection")
        )
        .subcommand(SubCommand::with_name("repocheck")
            .about("Runs beast for the commit range previously specified in the yaml set via 'beast config'.")
        )
        .get_matches();

    let mut config = AppConfig::init();

    // Handle subcommands
    handle_config_commands(&matches, &mut config);
    handle_database_commands(&matches, &config);
    handle_repocheck_commands(&matches, &config);

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

    // Plot last results
    if let Some(ref _matches) = matches.subcommand_matches("plotlast") {
        let last_results = parse::parse_cumulated_benchmark_file();
        plot_all(&last_results, plot_time_unit);
        return Ok(());
    }

    // Benchmark execution handling
    println!("Root scan directory: {:?}", root_dir.as_os_str());

    let mut benchmark_paths = find_executables(root_dir, filter_pattern);

    if benchmark_paths.is_empty() {
        println!("No benchmarks found to run!");
        return Ok(());
    }

    benchmark_paths.sort();

    if matches.is_present("list") {
        println!("Found benchmark executables:\n");
        println!(
            "{}",
            benchmark_paths
                .iter()
                .fold(String::new(), |total_str, arg| total_str
                    + &arg.as_path().to_string_lossy()
                    + "\n")
        );
        return Ok(());
    }

    let benchmark_results = execute_benchmarks(benchmark_paths);
    if !matches.is_present("noplot") {
        plot_all(&benchmark_results, plot_time_unit);
    }

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
        match matches.value_of("repocheck_yaml_path") {
            Some(yaml_path) => {
                match std::fs::canonicalize(yaml_path) {
                    Ok(path) => config
                        .set_repocheck_config_yaml(&path.as_path().to_string_lossy().to_string()),
                    Err(e) => log::error!(
                        "Path '{}' does not exist or can't be read ({})!",
                        yaml_path,
                        e
                    ),
                };
            }
            None => {}
        }
        std::process::exit(0);
    }
}

fn handle_database_commands(matches: &ArgMatches, config: &AppConfig) {
    let plot_time_unit = matches.value_of("timeunit").unwrap();

    if let Some(ref submatches) = matches.subcommand_matches("dbpush") {
        if config.is_db_config_set() {
            let db = DataBase::init(&config);
            let tag_option = submatches.value_of("tag").map(String::from);
            db.push_last_results(tag_option);
        } else {
            print_config_not_set();
        }
        std::process::exit(0);
    }
    if let Some(ref submatches) = matches.subcommand_matches("dbplot") {
        if config.is_db_config_set() {
            let filter_pattern = submatches.value_of("fetchfilter").unwrap_or(".*");
            let db = DataBase::init(&config);

            let results = db.fetch(EntryFilter::ExeName(filter_pattern.to_string()));
            //TODO: add support for "tag" + "both"

            if results.is_empty() {
                println!("Did not find any matching results. Nothing to plot!");
                std::process::exit(0);
            }

            plot_db_entries(&results, plot_time_unit);
        } else {
            print_config_not_set();
        }
        std::process::exit(0);
    }
    if let Some(ref _submatches) = matches.subcommand_matches("dblist") {
        if config.is_db_config_set() {
            let db = DataBase::init(&config);
            let tags = db.list_tags();
            print!("\nFound tags:\n{:?}\n", tags);
        }
        std::process::exit(0);
    }
}

fn handle_repocheck_commands(matches: &ArgMatches, config: &AppConfig) {
    if let Some(ref _submatches) = matches.subcommand_matches("repocheck") {
        let yaml_path = Path::new(config.repocheck_config_yaml());
        parse_repocheck_settings(yaml_path);
        std::process::exit(0);
    }
}

fn print_config_not_set() {
    println!("database config is not yet set. Use 'beast config' for this.");
}

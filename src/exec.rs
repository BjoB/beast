use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::path::PathBuf;
use std::process::Command;

use crate::parse;
use crate::plot::{BenchmarkPlot};

pub fn execute_benchmarks<PathList: AsRef<Vec<PathBuf>>>(exe_paths: PathList) {
    let exe_count = exe_paths.as_ref().len() as u64;
    let bar = ProgressBar::new(exe_count);
    let sty = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}");
    //.progress_chars("##-");
    bar.set_style(sty);

    let result_file_path = result_file_path();

    let result_file_path_str = result_file_path
        .to_str()
        .expect("Could not convert benchmark result file path to str!");

    let bm_plot = BenchmarkPlot::new();

    for exe_path in exe_paths.as_ref() {
        let exe_name = exe_path.as_path().file_name().unwrap();
        let exe_msg = format!("Executing benchmark \"{}\"...", exe_name.to_string_lossy());
        bar.set_message(&exe_msg);

        Command::new(exe_path)
            .arg(format!("--benchmark_out={}", result_file_path_str))
            .arg("--benchmark_out_format=json")
            .output()
            .expect("failed to execute process");

        parse::parse_benchmark_json(&result_file_path);
        bar.inc(1);
    }
    bar.finish();
    bm_plot.plot();
}

fn result_file_path() -> PathBuf {
    let mut temp_dir = env::temp_dir();
    temp_dir.push("beast_temp_benchmarkoutput.json");
    return temp_dir;
}

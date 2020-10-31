use crate::parse::*;

use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::fs::remove_file;
use std::path::PathBuf;
use std::process::Command;

pub fn execute_benchmarks<PathList: AsRef<Vec<PathBuf>>>(
    exe_paths: PathList,
) -> Vec<BenchmarkResults> {
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

    let mut bm_all_results: Vec<BenchmarkResults> = Vec::new();

    for exe_path in exe_paths.as_ref() {
        let exe_name = exe_path.as_path().file_name().unwrap();
        bar.set_message(&format!(
            "Executing benchmark \"{}\"...",
            exe_name.to_string_lossy()
        ));

        let benchmark_output = Command::new(exe_path)
            .arg(format!("--benchmark_out={}", result_file_path_str))
            .arg("--benchmark_out_format=json")
            .output()
            .expect("failed to execute process");

        assert!(
            benchmark_output.status.success(),
            "{} returned with exit code {}!",
            exe_name.to_string_lossy(),
            benchmark_output.status
        );

        let cur_bm_results = parse_single_benchmark_file(&result_file_path);

        remove_file(result_file_path.as_path()).expect(&format!(
            "Unable to remove benchmark result file \"{}\"!",
            result_file_path_str
        ));

        bm_all_results.push(cur_bm_results);

        bar.inc(1);
    }
    bar.finish();

    println!("Benchmark execution finished successfully!");

    export_cumulated_results(&bm_all_results);

    bm_all_results
}

fn result_file_path() -> PathBuf {
    let mut temp_dir = env::temp_dir();
    temp_dir.push("beast_temp_benchmarkoutput.json");
    return temp_dir;
}

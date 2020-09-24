use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::parse;
use crate::plot;

pub fn execute_benchmarks<PathList: AsRef<Vec<PathBuf>>>(exe_paths: PathList) {
    let exe_count = exe_paths.as_ref().len() as u64;
    let bar = ProgressBar::new(exe_count);
    let sty = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}");
    //.progress_chars("##-");
    bar.set_style(sty);

    for exe_path in exe_paths.as_ref() {
        let exe_name = exe_path.as_path().file_name().unwrap();
        let exe_msg = format!("Executing benchmark \"{}\"...", exe_name.to_string_lossy());
        bar.set_message(&exe_msg);
        Command::new(exe_path)
            .arg(format!("--benchmark_out={}", results_path_str()))
            .arg("--benchmark_out_format=json")
            .output()
            .expect("failed to execute process");
        parse::parse_benchmark_json(Path::new(results_path_str()));
        bar.inc(1);
    }
    bar.finish();
    plot::plot(); //TODO: just example
}

fn results_path_str() -> &'static str {
    return "/tmp/beast_benchmarkoutput.json";
}

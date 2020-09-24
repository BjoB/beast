use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
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
        let exe_output = Command::new(exe_path)
            .arg("--benchmark_format=json")
            .output()
            .expect("failed to execute process");
        //println!("{:?}", exe_output);
        parse::parse_benchmark_output(exe_output); //TODO: implement!
        bar.inc(1);
    }
    bar.finish();
    plot::plot(); //TODO: just example
}

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

const LAST_RESULTS_FILENAME: &str = "beast_temp_lastresults.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct DataBaseEntry {
    pub exe_name: String,
    pub tag: String,
    pub results: BenchmarkResults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BenchmarkResults {
    pub context: BenchmarkContext,
    pub benchmarks: Vec<BenchmarkResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BenchmarkContext {
    pub date: String,
    pub executable: PathBuf,
    pub num_cpus: i32,
    pub mhz_per_cpu: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: i32,
    pub real_time: f64,
    pub cpu_time: f64,
    pub time_unit: Option<String>,
}

pub fn parse_single_benchmark_file<P: AsRef<Path>>(file_path: P) -> BenchmarkResults {
    let json = json_from_file(file_path);
    let bm_context = json.get("context").unwrap();
    let bm_list = json["benchmarks"].as_array().unwrap();

    let mut results = BenchmarkResults {
        context: serde_json::from_value(bm_context.clone()).unwrap(),
        benchmarks: Vec::new(),
    };

    for single_bm in bm_list {
        let bm_result: BenchmarkResult = serde_json::from_value(single_bm.clone()).unwrap();
        // println!("{:?}", bm_result);
        results.benchmarks.push(bm_result);
    }

    return results;
}

pub fn parse_cumulated_benchmark_file() -> Vec<BenchmarkResults> {
    let file_path = exported_results_file_path();
    if !Path::exists(file_path.as_path()) {
        println!("No benchmark result file found! Run 'beast' to create one!");
        std::process::exit(0);
    }
    let cumulated_results = json_from_file(file_path);
    return serde_json::from_value(cumulated_results)
        .expect("Could not deserialize JsonValue from cumulated benchmark file!");
}

pub fn export_cumulated_results(cumulated_results: &Vec<BenchmarkResults>) {
    let export_file = exported_results_file_path();
    let export_file_path_str = export_file.as_path().to_string_lossy();

    let f = File::create(&export_file)
        .expect(&format!("Could not create file {}!", export_file_path_str));
    let results_json_val = json!(*cumulated_results);

    serde_json::to_writer(&f, &results_json_val).expect(&format!(
        "Could not write to file {}!",
        export_file_path_str
    ));
}

fn json_from_file<P: AsRef<Path>>(file_path: P) -> serde_json::Value {
    let result_file = File::open(file_path).expect("Benchmark result file not found!");
    let reader = BufReader::new(result_file);
    return serde_json::from_reader(reader).expect("JSON was not well-formatted!");
}

fn exported_results_file_path() -> PathBuf {
    let mut temp_dir = env::temp_dir();
    temp_dir.push(LAST_RESULTS_FILENAME);
    return temp_dir;
}

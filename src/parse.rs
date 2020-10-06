use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

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
    pub time_unit: String,
}

pub fn parse_benchmark_json<P: AsRef<Path>>(result_file_path: P) -> BenchmarkResults {
    let result_file = File::open(result_file_path).expect("Benchmark result file not found!");
    let reader = BufReader::new(result_file);
    let json: serde_json::Value =
        serde_json::from_reader(reader).expect("JSON was not well-formatted!");

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

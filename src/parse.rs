use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn parse_benchmark_json<P: AsRef<Path>>(result_file_path: P) {
    let result_file = File::open(result_file_path).expect("Benchmark result file not found!");
    let reader = BufReader::new(result_file);
    let json: serde_json::Value =
        serde_json::from_reader(reader).expect("JSON was not well-formatted!");

    let bm_list = json["benchmarks"].as_array().unwrap();

    for single_bm in bm_list {
        let bm_name = single_bm.get("name").unwrap();
        let bm_real_time = single_bm.get("cpu_time").unwrap();
    }

    //let serialized = serde_json::to_string(&_test).unwrap();
    //println!("{}", serialized);
}

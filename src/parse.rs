//use std::process::Output;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// pub fn parse_benchmark_output(output: Output) {
//     let output_as_str = String::from_utf8_lossy(&output.stdout);
//     //println!("{:?}", output);
//     let json: serde_json::Value =
//         serde_json::from_str(&output_as_str).expect("JSON was not well-formatted!");
//     let test = json.get("benchmarks").unwrap();
//     println!("Test");
// }

pub fn parse_benchmark_json<P: AsRef<Path>>(result_file_path: P) {
    let result_file = File::open(result_file_path).expect("Benchmark result file not found!");
    let reader = BufReader::new(result_file);
    let json: serde_json::Value =
        serde_json::from_reader(reader).expect("JSON was not well-formatted!");
    let _test = json.get("benchmarks").unwrap();
    let serialized = serde_json::to_string(&_test).unwrap();
    println!("{}", serialized);
}

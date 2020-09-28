use crate::parse::*;
use plotly::layout::{BarMode, Layout};
use plotly::{Bar, Plot};

// pub struct BenchmarkPlot {
//     bm_list: Vec<Vec<String>>,
//     value_list: Vec<Vec<f32>>,
// }

// impl BenchmarkPlot {
//     pub fn new() -> BenchmarkPlot {
//         BenchmarkPlot {
//             bm_list: Vec::new(),
//             value_list: Vec::new(),
//         }
//     }

//     pub fn add_benchmark_infos(&self, context: serde_json::Value) {}

//     pub fn add_benchmark_result(&self, bm_result: serde_json::Value) {
//         //self.bmplot.add_trace();
//     }

//     pub fn plot(&self) {
//         let benchmarks1 = vec!["bm11", "bm12"];
//         let trace1 = Bar::new(benchmarks1, vec![20, 14]).name("prime500");

//         let benchmarks2 = vec!["bm21", "bm22"];
//         let trace2 = Bar::new(benchmarks2, vec![12, 18]).name("prim1000");

//         let layout = Layout::new().bar_mode(BarMode::Group);
//         let mut plot = Plot::new();
//         plot.add_trace(trace1);
//         plot.add_trace(trace2);
//         plot.set_layout(layout);
//         plot.show();
//     }
// }

pub fn plot_all(all_results: &Vec<BenchmarkResults>) {
    let layout = Layout::new().bar_mode(BarMode::Group);
    let mut plot = Plot::new();
    plot.set_layout(layout);
    for bm_results in all_results {
        let mut sub_benchmark_names = vec![];
        let mut sub_benchmark_cpu_times = vec![];
        let bm_results_name = bm_results.context.executable.as_path().file_name().unwrap();

        // copy sub benchmarks names for trace
        for sub_bm_res in &bm_results.benchmarks {
            sub_benchmark_names.push(sub_bm_res.name.clone());
            sub_benchmark_cpu_times.push(sub_bm_res.cpu_time);
        }

        let res_trace = Bar::new(sub_benchmark_names, sub_benchmark_cpu_times)
            .name(&bm_results_name.to_string_lossy());

        plot.add_trace(res_trace);
    }

    plot.show();
}

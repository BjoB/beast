use plotly::layout::{BarMode, Layout};
use plotly::{Bar, Plot};

pub struct BenchmarkPlot {
    bm_list: Vec<Vec<String>>,
    value_list: Vec<Vec<f32>>,
}

impl BenchmarkPlot {
    pub fn new() -> BenchmarkPlot {
        BenchmarkPlot {
            bm_list: Vec::new(),
            value_list: Vec::new(),
        }
    }

    pub fn add_benchmark_infos(&self, context: serde_json::Value) {}

    pub fn add_benchmark_result(&self, bm_result: serde_json::Value) {
        //self.bmplot.add_trace();
    }

    pub fn plot(&self) {
        let benchmarks1 = vec!["bm11", "bm12"];
        let trace1 = Bar::new(benchmarks1, vec![20, 14]).name("prime500");

        let benchmarks2 = vec!["bm21", "bm22"];
        let trace2 = Bar::new(benchmarks2, vec![12, 18]).name("prim1000");

        let layout = Layout::new().bar_mode(BarMode::Group);
        let mut plot = Plot::new();
        plot.add_trace(trace1);
        plot.add_trace(trace2);
        plot.set_layout(layout);
        plot.show();
    }
}

use crate::parse::*;

use plotly::common::{DashType, Line, LineShape, Mode, Title};
use plotly::layout::{Axis, BarMode, Layout};
use plotly::{Bar, Plot, Scatter};
use std::collections::HashMap;
use std::time::Duration;

pub fn plot_all_as_bars(all_results: &Vec<BenchmarkResults>, plot_time_unit: &str) {
    // use first benchmark for cpu info as all results are retrieved on the same machine
    let plot_title = format!(
        "CPU count: {}, MHz/CPU: {}",
        all_results[0].context.num_cpus, all_results[0].context.mhz_per_cpu
    )
    .to_string();

    let y_title = format!("CPU runtime [{}]", plot_time_unit).to_string();
    let mut plot = Plot::new();

    let layout = Layout::new()
        .title(Title::from(plot_title.as_str()))
        .bar_mode(BarMode::Group)
        .bar_group_gap(0.1)
        .x_axis(Axis::new().auto_margin(true))
        .y_axis(Axis::new().title(Title::from(y_title.as_str())));

    plot.set_layout(layout);

    for bm_results in all_results {
        let mut sub_bm_names = vec![];
        let mut sub_bm_cpu_times = vec![];
        let bm_results_name = bm_results.context.executable.as_path().file_name().unwrap();

        // collect sub benchmarks results for trace
        for sub_bm_res in &bm_results.benchmarks {
            let sub_bm_duration =
                from_benchmark_time(sub_bm_res.time_unit.as_ref(), sub_bm_res.cpu_time as u64);
            let sub_bm_converted_cpu_time = convert_time_to_unit(sub_bm_duration, plot_time_unit);

            sub_bm_names.push(sub_bm_res.name.clone());
            sub_bm_cpu_times.push(sub_bm_converted_cpu_time);
        }

        plot.add_trace(
            Bar::new(sub_bm_names, sub_bm_cpu_times).name(&bm_results_name.to_string_lossy()),
        );
    }

    plot.show();
}

pub fn plot_all_as_lines(all_results: &Vec<BenchmarkResults>, plot_time_unit: &str, x_title: &str) {
    // use first benchmark for cpu info as all results are retrieved on the same machine
    let plot_title = format!(
        "CPU count: {}, MHz/CPU: {}",
        all_results[0].context.num_cpus, all_results[0].context.mhz_per_cpu
    )
    .to_string();
    let y_title = format!("CPU runtime [{}]", plot_time_unit).to_string();
    let mut plot = Plot::new();

    for bm_results in all_results {
        let mut x_values: HashMap<String, Vec<u64>> = HashMap::new();
        let mut y_values: HashMap<String, Vec<f64>> = HashMap::new();
        // let bm_results_name = bm_results.context.executable.as_path().file_name().unwrap();

        // collect sub benchmarks results for traces
        for sub_bm_res in &bm_results.benchmarks {
            let splitted_name = sub_bm_res.name.split("/");
            let splitted_name_vec = splitted_name.collect::<Vec<&str>>();
            let sub_bm_name = splitted_name_vec[0];
            let sub_bm_x_val = splitted_name_vec[1].parse::<u64>().unwrap();

            x_values
                .entry(sub_bm_name.to_string())
                .or_insert(Vec::new())
                .push(sub_bm_x_val);

            let sub_bm_duration =
                from_benchmark_time(sub_bm_res.time_unit.as_ref(), sub_bm_res.cpu_time as u64);
            let sub_bm_converted_cpu_time = convert_time_to_unit(sub_bm_duration, plot_time_unit);

            y_values
                .entry(sub_bm_name.to_string())
                .or_insert(Vec::new())
                .push(sub_bm_converted_cpu_time);
        }

        for bm_name in y_values.keys() {
            let trace = Scatter::new(x_values[bm_name].to_owned(), y_values[bm_name].to_owned())
                .mode(Mode::LinesMarkers)
                .name(bm_name)
                .line(Line::new().dash(DashType::Dash));
            plot.add_trace(trace);
        }
    }

    let layout = Layout::new()
        .title(Title::from(plot_title.as_str()))
        .x_axis(Axis::new().title(Title::from(x_title)))
        .y_axis(Axis::new().title(Title::from(y_title.as_str())));
    // TODO: create common tick list from all x-value-vectors (use tick_mode(TickMode::Array).tick_values())

    plot.set_layout(layout);
    plot.show();
}

// TODO: refine plot options, e.g. include means and variances in chart
pub fn plot_all_as_commit_series(results: &Vec<BenchmarkResults>, plot_time_unit: &str) {
    let mut xlabels: HashMap<String, Vec<String>> = HashMap::new();
    let mut cpu_times: HashMap<String, Vec<f64>> = HashMap::new();
    let mut tags: HashMap<String, Vec<String>> = HashMap::new();

    // collect benchmark data per commit for each "exename_benchmarkname"
    for result in results {
        for benchmark in &result.benchmarks {
            let exe_name = result.context.executable.file_name().unwrap();
            let trace_name = exe_name.to_string_lossy().to_owned() + "_" + benchmark.name.as_str();
            let trace_name = trace_name.to_string();

            // build current xlabel
            let commit_id = result.commit.as_ref().unwrap();
            let xlabel = build_label(commit_id.as_str(), "");

            // set cpu_time based on time unit for plot
            let cpu_time_as_duration =
                from_benchmark_time(benchmark.time_unit.as_ref(), benchmark.cpu_time as u64);
            let converted_cpu_time = convert_time_to_unit(cpu_time_as_duration, plot_time_unit);

            xlabels
                .entry(trace_name.clone())
                .or_insert(Vec::new())
                .push(xlabel);
            cpu_times
                .entry(trace_name.clone())
                .or_insert(Vec::new())
                .push(converted_cpu_time);
            tags.entry(trace_name.clone())
                .or_insert(Vec::new())
                .push(commit_id.clone());
        }
    }

    // create plot
    let y_title = format!("CPU runtime [{}]", plot_time_unit).to_string();

    let layout = Layout::new()
        .title(Title::from("Benchmark results over time"))
        .x_axis(Axis::new().title(Title::from("Commit")).auto_margin(true))
        .y_axis(Axis::new().title(Title::from(y_title.as_str())));

    let mut plot = Plot::new();

    plot.set_layout(layout);

    for trace_name in xlabels.keys() {
        let trace = Scatter::new(xlabels[trace_name].clone(), cpu_times[trace_name].clone())
            .mode(Mode::LinesMarkers)
            .name(trace_name)
            .text_array(tags[trace_name].clone())
            .line(Line::new().shape(LineShape::Hv));

        plot.add_trace(trace);
    }

    plot.show();
}

pub fn plot_db_entries(db_entries: &Vec<DataBaseEntry>, plot_time_unit: &str) {
    let mut xlabels: HashMap<String, Vec<String>> = HashMap::new();
    let mut cpu_times: HashMap<String, Vec<f64>> = HashMap::new();
    let mut tags: HashMap<String, Vec<String>> = HashMap::new();

    // collect time series data for each "exename_benchmarkname"
    for db_entry in db_entries {
        for single_result in &db_entry.results.benchmarks {
            let trace_name = db_entry.exe_name.clone() + "_" + single_result.name.as_str();

            // build current xlabel
            let xlabel = build_label(db_entry.results.context.date.as_str(), "");

            // set cpu_time based on time unit for plot
            let cpu_time_as_duration = from_benchmark_time(
                single_result.time_unit.as_ref(),
                single_result.cpu_time as u64,
            );
            let converted_cpu_time = convert_time_to_unit(cpu_time_as_duration, plot_time_unit);

            xlabels
                .entry(trace_name.clone())
                .or_insert(Vec::new())
                .push(xlabel);
            cpu_times
                .entry(trace_name.clone())
                .or_insert(Vec::new())
                .push(converted_cpu_time);
            tags.entry(trace_name.clone())
                .or_insert(Vec::new())
                .push(db_entry.tag.clone());
        }
    }

    // create plot
    let y_title = format!("CPU runtime [{}]", plot_time_unit).to_string();

    let layout = Layout::new()
        .title(Title::from("Benchmark results over time"))
        .x_axis(Axis::new().title(Title::from("Date")).auto_margin(true))
        .y_axis(Axis::new().title(Title::from(y_title.as_str())));

    let mut plot = Plot::new();

    plot.set_layout(layout);

    for trace_name in xlabels.keys() {
        let trace = Scatter::new(xlabels[trace_name].clone(), cpu_times[trace_name].clone())
            .mode(Mode::LinesMarkers)
            .name(trace_name)
            .text_array(tags[trace_name].clone())
            .line(Line::new().shape(LineShape::Hv));

        plot.add_trace(trace);
    }

    plot.show();
}

fn from_benchmark_time(from_time_unit: Option<&String>, time: u64) -> Duration {
    match from_time_unit {
        Some(from_time_unit) => match from_time_unit.as_ref() {
            "ns" => Duration::from_nanos(time),
            "us" => Duration::from_micros(time),
            "ms" => Duration::from_millis(time),
            _ => panic!("Unknown time unit provided!"),
        },
        None => {
            println!("No time unit was provided. Assuming ns!");
            Duration::from_nanos(time)
        }
    }
}

fn convert_time_to_unit(duration: Duration, time_unit: &str) -> f64 {
    let converted_time = match time_unit {
        "ns" => duration.as_nanos(),
        "us" => duration.as_micros(),
        "ms" => duration.as_millis(),
        _ => panic!("Unknown time unit provided!"),
    };
    converted_time as f64
}

fn build_label(date_time: &str, tag: &str) -> String {
    match tag {
        "" => date_time.to_string(),
        _ => String::new() + date_time + " (" + tag + ")",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_label_function() {
        assert_eq!(build_label("Test", "123"), "Test (123)");
        assert_eq!(build_label("Test", ""), "Test");
    }
}

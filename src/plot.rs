use plotly::{Bar, Plot};

pub fn plot() {
    let animals = vec!["giraffes", "orangutans", "monkeys"];
    let t = Bar::new(animals, vec![20, 14, 23]);
    let mut plot = Plot::new();
    plot.add_trace(t);
    plot.show();
    //println!("{}", plot.to_inline_html(Some("basic_bar_chart")));
}

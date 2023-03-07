use druid::widget::{Flex, TextBox, SvgData, Svg, Slider};
use druid::{
    widget::{Label, Split},
    AppLauncher, Widget, WindowDesc,
};
use druid::{Data, Lens, WidgetExt};
use graph::{Graph, linspace};
use plotters::prelude::{ChartBuilder, Rectangle};
use plotters::series::LineSeries;
use plotters::style::{FontDesc, IntoTextStyle, RGBColor, WHITE, ShapeStyle, RED};
use plotters_druid::Plot;

mod graph;
mod shunting;

#[derive(Debug, Clone, Data, Lens)]
struct State {
    graph: Graph,
    num_rect: i32,
    xistar: f64,
    area: f32,
}

fn build_options() -> impl Widget<State> {
    let eq_option = Flex::row()
        .with_child(Label::new("Graph Eq:"))
        .with_child(TextBox::new().lens(Graph::func_string).lens(State::graph));

    let xistar_asset = include_str!("assets/xistar.svg").parse::<SvgData>().unwrap();

    let xi_option = Flex::row()
        .with_child(Svg::new(xistar_asset))
        .with_child(Slider::new().lens(State::xistar));

    Flex::column()
        .with_child(eq_option)
        .with_child(xi_option)
}

fn build_plot() -> impl Widget<State> {
    Plot::new(|(_w, _h), data: &State, root| {

        let points = match data.graph.get_series() {
            Ok(x) => x,
            Err(_) => return
        };

        let mut y_sorted = points.iter().cloned().map(|x| x.1).collect::<Vec<_>>();
        y_sorted.sort_by(|a, b| a.total_cmp(b));

        let font = FontDesc::new(
            plotters::style::FontFamily::SansSerif,
            15.,
            plotters::style::FontStyle::Normal,
        );

        // create chart + options

        let x_spec = data.graph.domain.0..data.graph.domain.1;
        // let y_spec = data.graph.range.0..data.graph.range.1;
        let y_spec = y_sorted.first().unwrap_or(&0.0).to_owned()..y_sorted.last().unwrap_or(&10.0).to_owned();
        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_spec, y_spec)
            .unwrap();

        // draw grid + axis
        chart
            .configure_mesh()
            .axis_style(&RGBColor(28, 28, 28))
            .x_label_style(font.clone().with_color(&WHITE))
            .y_label_style(font.clone().with_color(&WHITE))
            .draw()
            .unwrap();

        // draw graph
        chart
            .draw_series(LineSeries::new(points, &WHITE))
            .unwrap();
        // if let Ok(points) = points_res {
        //     chart
        //         .draw_series(LineSeries::new(points, &WHITE))
        //         .unwrap();
        // }

        // draw rectangles
        // let area = chart.plotting_area();

        // for a in linspace(data.graph.domain.0, data.graph.domain.1, data.num_rect + 1).windows(2) {
        //     let l = match data.graph.f(a[0]) {Ok(x) => x, _ => break};
        //     let r = match data.graph.f(a[1]) {Ok(x) => x, _ => break};
        //     let top_right = match data.graph.f(a[0] + data.xistar as f32 * (r - l)) {Ok(x) => x, _ => break};

        //     area.draw(&Rectangle::new([
        //         (a[0], top_right),
        //         (a[1], 0.0)
        //     ], ShapeStyle {
        //         color: RED.into(),
        //         filled: false,
        //         stroke_width: 1,
        //     })).unwrap();
        // }
    })
}

fn build_root() -> impl Widget<State> {
    Split::columns(build_options(), build_plot()).split_point(0.27)
}

fn main() {
    let window = WindowDesc::new(build_root())
        .window_size((1050.0, 650.0))
        .title("Reimann Visualizer");

    let initial_data = State {
        graph: Graph{
            func_string: "x^2".to_string(),
            number_points: 100,
            domain: (0.0, 10.0),
            // range: (0.0, 10.0),
        },
        num_rect: 4,
        xistar: 1.0,
        area: 0.0,
    };

    AppLauncher::with_window(window)
        .launch(initial_data)
        .unwrap();
}

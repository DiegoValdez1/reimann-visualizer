use druid::widget::{Flex, TextBox, SvgData, Svg, Slider};
use druid::{
    widget::{Label, Split},
    AppLauncher, Widget, WindowDesc,
};
use druid::{Data, Lens, WidgetExt};
use graph::Graph;
use plotters::prelude::ChartBuilder;
use plotters::series::LineSeries;
use plotters::style::{FontDesc, IntoTextStyle, RGBColor, WHITE};
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
        let font = FontDesc::new(
            plotters::style::FontFamily::SansSerif,
            16.,
            plotters::style::FontStyle::Normal,
        );

        // create chart + options
        let x_spec = data.graph.domain.0..data.graph.domain.1;
        let y_spec = data.graph.range.0..data.graph.range.1;
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
        let points_res = data.graph.get_series();
        if let Ok(points) = points_res {
            chart
                .draw_series(LineSeries::new(points, &WHITE))
                .unwrap();
        }
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
            number_points: 1000,
            domain: (-10.0, 10.0),
            range: (-10.0, 10.0),
        },
        num_rect: 0,
        xistar: 1.0,
        area: 0.0,
    };

    AppLauncher::with_window(window)
        .launch(initial_data)
        .unwrap();
}

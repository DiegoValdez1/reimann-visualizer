use druid::widget::{Button, Flex, Slider, SvgData, Svg};
use druid::{
    widget::{Label, Split},
    AppLauncher, Widget, WindowDesc,
};
use druid::{Data, Lens, WidgetExt};
use druid_widget_nursery::DropdownSelect;
use graph::{linspace, Graph, Gtype};
use plotters::prelude::{ChartBuilder, Rectangle};
use plotters::series::LineSeries;
use plotters::style::{FontDesc, IntoTextStyle, RGBColor, ShapeStyle, RED, WHITE};
use plotters_druid::Plot;

mod graph;

#[derive(Debug, Clone, Data, Lens)]
struct State {
    graph: Graph,
    num_rect: i32,
    xistar: f64
}

impl State {
    fn calculate_area(&self) -> f32 {
        // THIS IS WRONG SO WRONG BUT ITS GOOD ENOUGH FOR PROJECT

        // there has to be some better way to do this.
        // TODO: make this not shit

        let mut area = 0.0;
        let points = self.graph.series();
        for partition in linspace(points.first().unwrap().0, points.last().unwrap().0, self.num_rect + 1).windows(2) {
            let eq = &self.graph.gtype;
            let l = partition[0];
            let r = partition[1];
            let height = eq.f(l + self.xistar as f32 * (r - l));
            area += height * (r - l);

        }

        area
    }
}

fn build_options() -> impl Widget<State> {
    let options_graph = Flex::row()
        .with_child(Label::new("Graph EQ: "))
        .with_child(DropdownSelect::new(vec![("y = x", Gtype::Basic), ("y = x^2", Gtype::Exponential)]).lens(Graph::gtype).lens(State::graph));

    let options_num_rect = Flex::row()
        .with_child(
            Button::new("Add Rectangle").on_click(|_, data: &mut State, _| {
                data.num_rect += 1;
            }),
        )
        .with_child(
            Button::new("Sub Rectangle").on_click(|_, data: &mut State, _| {
                if data.num_rect > 0 {
                    data.num_rect -= 1;
                }
            }),
        );

    let asset_xistar = include_str!("assets/xistar.svg").parse::<SvgData>().expect("Could not find 'xistar.svg' asset file.");
    let options_xistar = Flex::row()
        .with_child(Svg::new(asset_xistar))
        .with_child(Slider::new().lens(State::xistar));

    Flex::column()
        .with_child(options_graph)
        .with_child(Label::dynamic(|data: &State, _| {
            format!("Area under graph: {}", data.graph.gtype.area(data.graph.domain))
        }))
        .with_child(Label::dynamic(|data: &State, _| {
            format!("Area of rectangles = {}", data.calculate_area())
        }))
        .with_child(Label::dynamic(|data: &State, _| format!("# of rectangles: {}", data.num_rect)))
        .with_child(options_num_rect)
        .with_child(options_xistar)
}

fn build_plot() -> impl Widget<State> {
    Plot::new(|(_w, _h), data: &State, root| {
        let font = FontDesc::new(
            plotters::style::FontFamily::SansSerif,
            16.,
            plotters::style::FontStyle::Normal,
        );

        // create chart + options
        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(data.graph.domain.0..data.graph.domain.1, data.graph.range.0..data.graph.range.1)
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
        let points = data.graph.series();
        chart
            .draw_series(LineSeries::new(points.clone(), &WHITE))
            .unwrap();

        // draw rectangles
        let area = chart.plotting_area();
    
        for partition in linspace(points.first().unwrap().0, points.last().unwrap().0, data.num_rect + 1).windows(2) {
            let eq = &data.graph.gtype;
            let l = partition[0];
            let r = partition[1];

            let height = eq.f(l + data.xistar as f32 * (r - l));

            area.draw(&Rectangle::new([
                (l, height),
                (r, 0.0)  // 0.0 here is the x axis
            ], ShapeStyle {
                    color: RED.into(),
                    filled: false,
                    stroke_width: 1,
            })).unwrap();
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
        graph: Graph {
            gtype: Gtype::Exponential,
            domain: (-10.0, 10.0),
            range: (-10.0, 10.0)
        },
        num_rect: 0,
        xistar: 1.0
    };

    AppLauncher::with_window(window)
        .launch(initial_data)
        .unwrap();
}

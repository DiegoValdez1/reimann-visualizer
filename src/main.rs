use druid::widget::{Button, Flex, Slider};
use druid::{
    widget::{Label, Split},
    AppLauncher, Widget, WindowDesc,
};
use druid::{Data, Lens, WidgetExt};
use graph::{linspace, Graphs};
use plotters::prelude::{ChartBuilder, Rectangle};
use plotters::series::LineSeries;
use plotters::style::{FontDesc, IntoTextStyle, RGBColor, ShapeStyle, RED, WHITE};
use plotters_druid::Plot;

mod graph;

#[derive(Debug, Clone, Data, Lens)]
struct State {
    graph: Graphs,
    num_rect: i32,
    xistar: f64,
    area: f32,
    text: String,
}

impl State {
    fn calculate_area(&self) -> f32 {
        let domain = (0f32, 10f32);
        let delta_x = (domain.1 - domain.0) / self.num_rect as f32;
        let mut area = 0f32;

        for i in 0..self.num_rect {
            let xistar = i as f32 * delta_x + self.xistar as f32 * delta_x;
            area += self.graph.get_y(xistar) * delta_x;
        }

        if self.num_rect == 0 {
            0.0
        } else {
            area
        }
    }
}

fn build_options() -> impl Widget<State> {
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

    let options_xistar = Flex::row()
        .with_child(Label::new("xistar: "))
        .with_child(Slider::new().lens(State::xistar));

    Flex::column()
        .with_spacer(0.50)
        .with_child(Label::dynamic(|data: &State, _| {
            format!("Area under graph: {}", data.graph.get_area())
        }))
        .with_child(Label::dynamic(|data: &State, _| {
            format!("Area of rectangles = {}", data.calculate_area())
        }))
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
            .x_label_area_size(25)
            .y_label_area_size(25)
            .build_cartesian_2d(0f32..10.0, 0f32..10.0)
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
        let line_points = data.graph.get_points();
        chart
            .draw_series(LineSeries::new(line_points, &WHITE))
            .unwrap();

        // draw rectangles
        let area = chart.plotting_area();

        for a in linspace(0.0, 10.0, data.num_rect + 1).windows(2) {
            // a[0] = left endpoint
            // a[1] = right endpoint
            area.draw(&Rectangle::new(
                [
                    (
                        a[0],
                        a[0] + data.xistar as f32 * data.graph.get_y(a[1] - a[0]),
                    ),
                    (a[1], 0.0),
                ],
                ShapeStyle {
                    color: RED.into(),
                    filled: false,
                    stroke_width: 1,
                },
            ))
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
        graph: Graphs::Basic,
        num_rect: 0,
        xistar: 1.0,
        area: 0.0,
        text: "".to_string(),
    };

    AppLauncher::with_window(window)
        .launch(initial_data)
        .unwrap();
}

use druid::widget::{Button, Flex, Slider, TextBox};
use druid::{
    widget::{Label, Split},
    AppLauncher, Widget, WindowDesc,
};
use druid::{Data, Lens, WidgetExt};
use graph::{linspace, Graph};
use plotters::prelude::{ChartBuilder, Rectangle};
use plotters::series::LineSeries;
use plotters::style::{FontDesc, IntoTextStyle, RGBColor, ShapeStyle, RED, WHITE};
use plotters_druid::Plot;
use shunting::ShuntingError;

mod graph;
mod shunting;

// check out field in druid lens

#[derive(Debug, Clone, Data, Lens)]
struct State {
    graph: Graph,
    num_rect: i32,
    xistar: f64,
    area: f32,
    text: String,
}

impl State {
    fn calculate_rect_area(&self) -> Result<f32, ShuntingError> {
        if self.num_rect == 0 {return Ok(0.0)}

        let domain = (0f32, 10f32);
        let delta_x = (domain.1 - domain.0) / self.num_rect as f32;
        let mut area = 0f32;

        for i in 0..self.num_rect {
            let xistar = i as f32 * delta_x + self.xistar as f32 * delta_x;
            area += self.graph.f(xistar)? * delta_x;
        }

        Ok(area)
    }
}

fn build_options() -> impl Widget<State> {
    let options_graph = Flex::row()
        .with_child(Label::new("Graph Eq:"))
        .with_child(TextBox::new().lens(Graph::func_string).lens(State::graph));

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
        .with_child(options_graph)
        // .with_child(Label::dynamic(|data: &State, _| {
            // format!("Area of rectangles = {}", data.calculate_rect_area().unwrap_or(0.0))
        // }))
        // .with_child(Label::dynamic(|data: &State, _| format!("# of rectangles: {}", data.num_rect)))
        // .with_child(options_num_rect)
        // .with_child(options_xistar)
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

        // draw rectangles
        // export this to a closure

        let area = chart.plotting_area();
        
        let draw_rect = |left: f32, right: f32| {

        };

        // for a in linspace(0.0, 10.0, data.num_rect + 1).windows(2) {
        //     let left = data.graph.get_y(a[0]);
        //     let right = data.graph.get_y(a[1]);
        //     area.draw(&Rectangle::new(
        //         [
        //             (
        //                 a[0],
        //                 a[0] + data.xistar as f32 * data.graph.f(right - left), // something wrong here with the exponential one
        //             ),
        //             (a[1], 0.0),
        //         ],
        //         ShapeStyle {
        //             color: RED.into(),
        //             filled: false,
        //             stroke_width: 1,
        //         },
        //     ))
        //     .unwrap();
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
            number_points: 1000,
            domain: (0.0, 10.0),
            range: (0.0, 10.0),
        },
        num_rect: 0,
        xistar: 1.0,
        area: 0.0,
        text: "".to_string(),
    };

    AppLauncher::with_window(window)
        .launch(initial_data)
        .unwrap();
}

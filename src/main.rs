#![allow(unused)] // get rid of this later

use druid::text::{ParseFormatter, Formatter};
use druid::widget::{Container, Button, Flex, TextBox, ValueTextBox};
use druid::{
    widget::{Label, Split},
    AppLauncher, Widget, WindowDesc,
};
use druid::{Data, WidgetExt, Lens};
use plotters::prelude::*;
use plotters_druid::Plot;

fn linspace(start: f32, stop: f32, amount: i32) -> Vec<f32> {
    (0..amount)
        .map(|x| x as f32)
        .map(|x| start + (x * (stop - start) / (amount - 1) as f32))
        .collect()
}

#[derive(Debug, Clone, Data, PartialEq)]
enum Graphs {
    Basic,
    BasicExponential
}

impl Graphs {
    fn get_y(&self, x: f32) -> f32 {
        match self {
            Graphs::Basic => x,
            Graphs::BasicExponential => todo!(),
        }
    }

    fn get_points(&self) -> impl Iterator<Item = (f32, f32)> {
        match self {
            Graphs::Basic => (0..=10).map(|x| x as f32).map(|x| (x, x)),
            Graphs::BasicExponential => todo!(),
        }
    }
}

#[derive(Debug, Clone, Data, Lens)]
struct State {
    graph: Graphs,
    num_rect: i32,
    xistar: f32,
    area: f32,
    text: String
}

impl State {
    fn calculate_area(&mut self) {
        // hardcoding y=x on [0, 10] here
        if self.num_rect == 0 {
            self.area = 0.0;
            return
        }

        let domain = (0f32, 10f32);
        let delta_x = (domain.1 - domain.0) / self.num_rect as f32;
        let area = 0f32;

        for i in 0..self.num_rect {
            let xistar = i as f32 * delta_x + self.xistar * delta_x;
            self.area = xistar * delta_x;  // replace xistar with f(xistar)
        }
    }
}

fn build_options() -> impl Widget<State> {
    let num_rect = Flex::row()
        .with_child(Button::new("Add Rectangle").on_click(|ctx, data: &mut State, _env| {
            data.num_rect += 1;
            data.calculate_area()
        }))
        .with_child(Button::new("Sub Rectangle").on_click(|ctx, data: &mut State, _env| {
            if data.num_rect > 0 {
                data.num_rect -= 1;
            }
            data.calculate_area()
        }));

    Flex::column()
        .with_spacer(0.50)
        .with_child(Label::dynamic(|data: &State, _env| format!("area of rectangles = {}", data.area)))
        .with_child(num_rect)
}

fn build_plot() -> impl Widget<State> {
    Plot::new(|(width, height), data: &State, root| {
        let font = FontDesc::new(FontFamily::SansSerif, 16., FontStyle::Normal);

        // create chart + options
        let mut chart = ChartBuilder::on(&root)
            // .caption("thing", ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(25)
            .y_label_area_size(25)
            .build_cartesian_2d(0f32..10.0, 0f32..10.0)
            .unwrap();

        // draw grid + axis
        chart.configure_mesh()
            .axis_style(&RGBColor(28, 28, 28))
            .x_label_style(font.clone().with_color(&WHITE))
            .y_label_style(font.clone().with_color(&WHITE))
            .draw()
            .unwrap();

        // draw graph
        // let line_points = (0..=10).map(|x| x as f32).map(|x| (x, x));
        let line_points = data.graph.get_points();
        chart
            .draw_series(LineSeries::new(line_points, &WHITE))
            .unwrap();

        // draw rectangles
        let area = chart.plotting_area();

        for a in linspace(0.0, 10.0, data.num_rect + 1).windows(2) {
            area.draw(&Rectangle::new(
                [(a[0], data.xistar * a[1]), (a[1], 0.0)], // the a[1] in the first tuple is xistar
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
    Split::columns(
        // Button::new("More Rectangles!").on_click(|_ctx, data: &mut State, _env| data.num_rect += 1),
        build_options(),
        build_plot(),
    )
    .split_point(0.27)
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
        text: "".to_string()
    };

    AppLauncher::with_window(window)
        .launch(initial_data)
        .unwrap();
}

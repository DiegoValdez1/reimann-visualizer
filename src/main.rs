#![allow(unused)] // get rid of this later

use druid::widget::{Container, Button};
use druid::{
    widget::{Label, Split},
    AppLauncher, Widget, WindowDesc,
};
use druid::{Data, WidgetExt};
use plotters::prelude::*;
use plotters_druid::Plot;

fn linspace(start: f32, stop: f32, amount: i32) -> Vec<f32> {
    (0..amount)
        .map(|x| x as f32)
        .map(|x| start + (x * (stop - start) / (amount - 1) as f32))
        .collect()
}

#[derive(Clone, Data)]
struct State {
    num_rect: i32,
    xistar: f32,
}

fn build_plot() -> impl Widget<State> {
    Plot::new(|(width, height), data: &State, root| {
        root.fill(&WHITE).unwrap();

        // create chart + options
        let mut chart = ChartBuilder::on(&root)
            // .caption("thing", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(25)
            .y_label_area_size(25)
            .build_cartesian_2d(0f32..10.0, 0f32..10.0)
            .unwrap();

        // draw grid + axis
        chart.configure_mesh().draw().unwrap();

        // draw graph
        let line_points = (0..=10).map(|x| x as f32).map(|x| (x, x));
        chart
            .draw_series(LineSeries::new(line_points.clone(), &BLACK))
            .unwrap();

        // draw rectangles
        let area = chart.plotting_area();

        for a in linspace(0.0, 10.0, data.num_rect + 1).windows(2) {
            area.draw(&Rectangle::new(
                [(a[0], data.xistar * a[1]), (a[1], 0.0)], // the a[1] in the first tuple is xistar
                ShapeStyle {
                    color: RED.into(),
                    filled: false,
                    stroke_width: 2,
                },
            ))
            .unwrap();
        }
    })
}

fn build_root() -> impl Widget<State> {
    Split::columns(
        // {
        //     let mut label = Label::new("hello world!");
        //     label.set_text_color(druid::Color::BLACK);
        //     label
        // },
        Button::new("More Rectangles!").on_click(|_ctx, data: &mut State, _env| data.num_rect += 1),
        build_plot(),
    )
    .split_point(0.33)
    .background(druid::Color::WHITE)
}

fn main() {
    let window = WindowDesc::new(build_root())
        .window_size((1050.0, 650.0))
        .title("Reimann Visualizer");

    let initial_data = State {
        num_rect: 4,
        xistar: 1.0,
    };

    AppLauncher::with_window(window)
        .launch(initial_data)
        .unwrap();
}

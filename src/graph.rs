use std::ops::Range;

use druid::Data;

#[derive(Debug, Clone, Data, PartialEq)]
pub enum Graphs {
    Basic,
    BasicExponential,
}

impl Graphs {
    pub fn get_y(&self, x: f32) -> f32 {
        match self {
            Graphs::Basic => x,
            Graphs::BasicExponential => x.powi(2),
        }
    }

    pub fn get_points(&self) -> Vec<(f32, f32)> {
        match self {
            Graphs::Basic => (0..=10).map(|x| x as f32).map(|x| (x, x)).collect(),
            Graphs::BasicExponential => {
                // x^2 = 10
                // x = sqrt(10)
                // let mut a = (0..=100).map(|x| (x as f32)/10f32).map(|x| (x, x.powi(2))).filter(|x| x.1 < 10f32).collect::<Vec<_>>();
                // a.push((10f32.sqrt(), 10f32));
                // a
                (0..=100).map(|x| (x as f32)/10f32).map(|x| (x, x.powi(2))).collect()
            }
        }
    }

    pub fn get_range(&self) -> Range<f32> {
        match self {
            Graphs::Basic => 0f32..10f32,
            Graphs::BasicExponential => 0f32..100f32,
        }
    }

    pub fn get_area(&self) -> f32 {
        match self {
            Graphs::Basic => 10f32.powi(2) / 2f32,
            Graphs::BasicExponential => 10f32.powi(3) / 3f32,
        }
    }
}

pub fn linspace(start: f32, stop: f32, amount: i32) -> Vec<f32> {
    (0..amount)
        .map(|x| x as f32)
        .map(|x| start + (x * (stop - start) / (amount - 1) as f32))
        .collect()
}

use druid::{Data, Lens};

use crate::shunting::{ShuntingError, solve};

#[derive(Debug, Clone, PartialEq, Data, Lens)]
pub struct Graph {
    // I need to do shunting yard every time I need equation
    // Can't think of a way to save it because I need to use the equation when I only have reference to the graph
    // Maybe try memoization?

    pub func_string: String,
    pub number_points: i32,
    pub domain: (f32, f32),
    // pub range: (f32, f32)
}

impl Graph {
    pub fn f(&self, x: f32) -> Result<f32, ShuntingError> {
        solve(&self.func_string, Some(x))
    }

    pub fn get_series(&self) -> Result<Vec<(f32, f32)>, ShuntingError> {
        let xpoints = linspace(self.domain.0, self.domain.1, self.number_points);
        let mut points: Vec<(f32, f32)> = Vec::new();

        for x in xpoints {
            points.push((x, self.f(x)?))
        }

        // for x in xpoints {
        //     let p = (x, self.f(x)?);
        //     if p.1 > self.range.1 || p.1 < self.range.0 {
        //         break
        //     } else {
        //         points.push(p)
        //     }
        // }

        Ok(points)
    }
}

pub fn linspace(start: f32, stop: f32, amount: i32) -> Vec<f32> {
    (0..amount)
        .map(|x| x as f32)
        .map(|x| start + (x * (stop - start) / (amount - 1) as f32))
        .collect()
}
use druid::{Data, Lens};

use crate::{shunting::{ShuntingError, solve}};

#[derive(Debug, Clone, PartialEq, Data, Lens)]
pub struct Graph {
    // im just gonna have to do shunting yard on the func_string every time I need the equation
    // becuase I cant think of a way check if the func_string will be equal
    // also there will be complications trying to change the function when I only have a reference to Graph
    // maybe memoization?

    pub func_string: String,
    pub number_points: i32,
    pub domain: (f32, f32),
    pub range: (f32, f32)
}

impl Graph {
    pub fn f(&self, x: f32) -> Result<f32, ShuntingError> {
        solve(&self.func_string, Some(x))
    }

    pub fn get_series(&self) -> Result<Vec<(f32, f32)>, ShuntingError> {
        let xpoints = linspace(self.domain.0, self.domain.1, self.number_points);
        let mut points: Vec<(f32, f32)> = Vec::new();

        for x in xpoints {
            let point = (x, self.f(x)?);
            if point.1 <= self.range.1 {
                points.push(point);
            } else {
                break
            }
        }

        // maybe to make this look better is to calculate slope from the last two points and use 
        // that to push a point with y = range.max with x being found from the above slope

        

        // i cant use this because on straight lines it fucks it up. I have to use the above idea.
        // let last = points.last().ok_or(ShuntingError::Other)?.clone();
        // points.push((last.0, self.range.1));

        // maybe I can check for y points that are above the range and only add the the last point like above if it exists.


        // let las = points.last().unwrap();
        // let sec = points.get(points.len()-2).unwrap();

        // let hypotenuse = ((las.0 - sec.0).powi(2) + (las.1 - las.0).powi(2)).sqrt();
        // dbg!(hypotenuse);


        Ok(points)
    }
}


// pub enum Graphs {
//     Basic,
//     BasicExponential,
// }

// impl Graphs {
//     pub fn get_y(&self, x: f32) -> f32 {
//         match self {
//             Graphs::Basic => x,
//             Graphs::BasicExponential => x.powi(2),
//         }
//     }

//     pub fn get_points(&self) -> Vec<(f32, f32)> {
//         match self {
//             Graphs::Basic => (0..=10).map(|x| x as f32).map(|x| (x, x)).collect(),
//             Graphs::BasicExponential => {
//                 // x^2 = 10
//                 // x = sqrt(10)
//                 // let mut a = (0..=100).map(|x| (x as f32)/10f32).map(|x| (x, x.powi(2))).filter(|x| x.1 < 10f32).collect::<Vec<_>>();
//                 // a.push((10f32.sqrt(), 10f32));
//                 // a
//                 (0..=100).map(|x| (x as f32)/10f32).map(|x| (x, x.powi(2))).collect()
//             }
//         }
//     }

//     pub fn get_range(&self) -> Range<f32> {
//         match self {
//             Graphs::Basic => 0f32..10f32,
//             Graphs::BasicExponential => 0f32..100f32,
//         }
//     }

//     pub fn get_area(&self) -> f32 {
//         match self {
//             Graphs::Basic => 10f32.powi(2) / 2f32,
//             Graphs::BasicExponential => 10f32.powi(3) / 3f32,
//         }
//     }
// }

pub fn linspace(start: f32, stop: f32, amount: i32) -> Vec<f32> {
    (0..amount)
        .map(|x| x as f32)
        .map(|x| start + (x * (stop - start) / (amount - 1) as f32))
        .collect()
}
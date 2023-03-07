use druid::{Data, Lens};

// this is horrible. So bounded. Never do this again. It works though, and I hate that it does.
#[derive(Debug, Clone, PartialEq, Data)]
pub enum Gtype {
    Basic,       // y = x
    Exponential  // y = x^2
}

impl Gtype {
    pub fn f(&self, x: f32) -> f32 {
        match self {
            Gtype::Basic => x,
            Gtype::Exponential => x.powi(2),
        }
    }

    pub fn reverse_f(&self, y: f32) -> f32 {
        match self {
            Gtype::Basic => y,
            Gtype::Exponential => y.sqrt(),
        }
    }

    pub fn area(&self, domain: (f32, f32)) -> f32 {
        // solve the integral for the given range
        match self {
            Gtype::Basic => domain.1.powi(2) / 2.0 - domain.0.powi(2) / 2.0,  // int(x) = (x^2)/2
            Gtype::Exponential => domain.1.powi(3) / 3.0 - domain.0.powi(3) / 3.0,  // int(x^2) = (x^3)/3
        }
    }
}

#[derive(Debug, Clone, PartialEq, Data, Lens)]
pub struct Graph {
    pub gtype: Gtype,
    pub domain: (f32, f32),
    pub range: (f32, f32)
}

impl Graph {
    pub fn series(&self, num_points: i32) -> Vec<(f32, f32)> {
        let xpoints = linspace(self.domain.0, self.domain.1, num_points);
        xpoints
            .into_iter()
            .map(|x| (x, self.gtype.f(x)))
            .filter(|x| x.1 < self.range.1)
            .filter(|x| x.1 > self.range.0)
            .collect::<Vec<_>>()
    }
}

pub fn linspace(start: f32, stop: f32, amount: i32) -> Vec<f32> {
    (0..amount)
        .map(|x| x as f32)
        .map(|x| start + (x * (stop - start) / (amount - 1) as f32))
        .collect()
}

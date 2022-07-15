use proconio::input;
use std::f64::consts::PI;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point {
    fn powi(&self, n: i32) -> Self {
        Self {
            x: self.x.powi(n),
            y: self.y.powi(n),
            z: self.z.powi(n),
        }
    }

    fn length(&self) -> f64 {
        let p2 = self.powi(2);
        (p2.x + p2.y + p2.z).sqrt()
    }
}

fn main() {
    input! {
        in_t: usize,
        in_l: i64,
        in_x: i64,
        in_y: i64,
        in_q: usize,
    }

    let p0 = Point {
        x: in_x as f64,
        y: in_y as f64,
        z: 0.0,
    };

    for _ in 0..in_q {
        input! {
            in_e: i64,
        }

        let theta = ((in_e % in_t as i64) as f64 / in_t as f64) * (2.0 * PI);

        let p1 = Point {
            x: 0.0,
            y: -theta.sin() * in_l as f64 / 2.0,
            z: (1.0 - theta.cos()) * in_l as f64 / 2.0,
        };

        let b = ((p1.x - p0.x).powi(2) + (p1.y - p0.y).powi(2)).sqrt();
        let c = (b / (p1 - p0).length()).acos() * 180.0 / PI;
        println!("{}", c);
    }
}

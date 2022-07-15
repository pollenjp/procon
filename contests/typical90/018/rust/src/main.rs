use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        in_t: usize,
        in_l: i64,
        in_x: i64,
        in_y: i64,
        in_q: usize,
    }

    let x0 = in_x as f64;
    let y0 = in_y as f64;
    let z0 = 0.0;

    for _ in 0..in_q {
        input! {
            in_e: i64,
        }

        let theta = ((in_e % in_t as i64) as f64 / in_t as f64) * (2.0 * PI);

        let x1 = 0.0;
        let y1 = -theta.sin() * in_l as f64 / 2.0;
        let z1 = (1.0 - theta.cos()) * in_l as f64 / 2.0;

        let a = (x1 - x0).powi(2) + (y1 - y0).powi(2) + (z1 - z0).powi(2);
        let b = (x1 - x0).powi(2) + (y1 - y0).powi(2);
        let c = (b / a).sqrt().acos() * 180.0 / PI;
        println!("{}", c);
    }
}

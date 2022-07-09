use proconio::input;

fn main() {
    input! {
        in_a: i32,
        in_b: i32,
        in_d: i32,
    }

    let a = in_a as f64;
    let b = in_b as f64;
    let d = std::f64::consts::PI * (in_d as f64) / 180.0;

    let rotate_matrix = vec![vec![d.cos(), -d.sin()], vec![d.sin(), d.cos()]];
    dbg!(&rotate_matrix);

    let new_a = rotate_matrix[0][0] * a + rotate_matrix[0][1] * b;
    let new_b = rotate_matrix[1][0] * a + rotate_matrix[1][1] * b;

    println!("{} {}", new_a, new_b);
}

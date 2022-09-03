use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_q: usize,
        in_x_vec: [(i64, i64); in_n],
        in_q_vec: [usize; in_q],
    };

    let mut rotated_vec = in_x_vec.clone();
    for i in 0..in_n {
        let (x, y) = rotated_vec[i];
        // rotate 45 degree
        // manhattan distance とするために座標を
        // 原点中心に sqrt(2) 倍する.
        // そのため cos(pi/2) = sin(pi/2) = 1 / sqrt(2) の係数が消える.
        rotated_vec[i] = (x - y, x + y);
    }

    let x_max = rotated_vec.iter().map(|&(x, _)| x).max().unwrap();
    let x_min = rotated_vec.iter().map(|&(x, _)| x).min().unwrap();
    let y_max = rotated_vec.iter().map(|&(_, y)| y).max().unwrap();
    let y_min = rotated_vec.iter().map(|&(_, y)| y).min().unwrap();

    for q in in_q_vec {
        let (x, y) = rotated_vec[q - 1];
        let &max_val = vec![
            (x - x_max).abs(),
            (x - x_min).abs(),
            (y - y_max).abs(),
            (y - y_min).abs(),
        ]
        .iter()
        .max()
        .unwrap();
        println!("{}", max_val);
    }
}

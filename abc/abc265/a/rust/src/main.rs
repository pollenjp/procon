use proconio::input;
fn main() {
    input! {
        in_x: usize,
        in_y: usize,
        in_n: usize,
    }
    if in_x <= in_y / 3 {
        // 全部 x 円で買う
        println!("{}", in_x * in_n);
    } else {
        let n_y = in_n / 3;
        let n_x = in_n % 3;
        println!("{}", in_y * n_y + in_x * n_x);
    }
}

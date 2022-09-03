use proconio::input;
fn main() {
    input! {
        in_n: usize,
        in_m: usize,
        in_x: usize,
        in_t: usize,
        in_d: usize,
    };

    if in_m > in_x {
        println!("{}", in_t);
    } else {
        println!("{}", in_t - in_d * (in_x - in_m));
    }
}

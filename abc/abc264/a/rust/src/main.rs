use proconio::input;
fn main() {
    input! {
        in_l: usize,
        in_r: usize,
    }

    let in_l = in_l - 1;
    let in_r = in_r - 1;

    println!(
        "{}",
        "atcoder"[in_l..in_r + 1]
            .chars()
            .collect::<Vec<char>>()
            .into_iter()
            .collect::<String>()
    );
}

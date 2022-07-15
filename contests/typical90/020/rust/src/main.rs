use proconio::input;

fn main() {
    input! {
        in_a: i64,
        in_b: i64,
        in_c: i64,
    }

    let left = in_a;
    let mut right: i64 = 1;

    for _ in 0..in_b {
        right *= in_c;
    }
    if right > left {
        println!("Yes");
    } else {
        println!("No");
    }
}

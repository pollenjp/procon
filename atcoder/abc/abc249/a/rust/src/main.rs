#![warn(clippy::pedantic, clippy::nursery)]

// in rust 1.60.0
// const fn calc_walked_length(x: i32, a: i32, b: i32, c: i32) -> i32 {
fn calc_walked_length(x: i32, a: i32, b: i32, c: i32) -> i32 {
    let tmp_quotient: i32 = x / (a + c);
    let tmp_reminder: i32 = x % (a + c);
    if tmp_reminder < a {
        return tmp_quotient * a * b + tmp_reminder * b;
    }
    return tmp_quotient * a * b + a * b;
}

fn main() {
    proconio::input! {
        in_a: i32,
        in_b: i32,
        in_c: i32,
        in_d: i32,
        in_e: i32,
        in_f: i32,
        in_x: i32,
    }

    // takahashi
    let takahashi_walk_length = calc_walked_length(in_x, in_a, in_b, in_c);
    // aoki
    let aoki_walk_length = calc_walked_length(in_x, in_d, in_e, in_f);

    match takahashi_walk_length.cmp(&aoki_walk_length) {
        std::cmp::Ordering::Less => println!("Aoki"),
        std::cmp::Ordering::Greater => println!("Takahashi"),
        std::cmp::Ordering::Equal => println!("Draw"),
    }
}

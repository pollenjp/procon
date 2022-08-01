use proconio::input;

fn main() {
    input! {
        in_y: i32,
    }

    let a = in_y / 4;
    let b = in_y % 4;

    if b <= 2 {
        println!("{}", 4 * a + 2);
    } else {
        println!("{}", 4 * (a + 1) + 2);
    }
}

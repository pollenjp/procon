#![warn(clippy::pedantic, clippy::nursery)]

fn main() {
    proconio::input! {
        in_s: String,
    }

    let num = 6 / in_s.len();
    for _ in 0..num {
        print!("{}", in_s);
    }
    println!("");
}

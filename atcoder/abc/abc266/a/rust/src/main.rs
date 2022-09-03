use proconio::input;

fn main() {
    input! {
        in_s: String
    }
    println!("{}", in_s.chars().nth(in_s.len() / 2).unwrap());
}

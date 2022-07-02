use proconio::input;

fn main() {
    input! {
        in_k: i32,
    }
    let hour = 21 + in_k / 60;
    let minute = in_k % 60;
    println!("{:>02}:{:>02}", hour, minute);
}

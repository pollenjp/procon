use proconio::input;

fn main() {
    input! {
        in_s: String,
    }
    let arr = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    let mut idx = 0;
    for (i, val) in arr.iter().enumerate() {
        if in_s == *val {
            idx = i;
            break;
        }
    }

    println!("{}", 5 - idx);
}

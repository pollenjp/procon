use proconio::input;

fn main() {
    input! {
        in_s: String,
        in_t: String,
    };

    if in_s.len() > in_t.len() {
        println!("No");
        return;
    }

    for i in 0..in_s.len() {
        if in_s[i..i + 1] != in_t[i..i + 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };

    let tkhs = a * 60 + b;
    let aok = c * 60 + d;

    if tkhs <= aok {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

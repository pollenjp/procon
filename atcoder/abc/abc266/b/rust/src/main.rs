use proconio::input;

static p: i64 = 998244353;

fn main() {
    input! {
        in_n: i64
    }

    let mut x = in_n % p;
    if x < 0 {
        x += p;
    }
    println!("{}", x);
}

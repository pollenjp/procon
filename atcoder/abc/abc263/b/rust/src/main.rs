use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_p: [usize; in_n-1],
    }

    let mut p = in_p[in_n - 2];
    let mut cnt = 1;
    while p != 1 {
        p = in_p[p - 2];
        cnt += 1;
    }

    println!("{}", cnt);
}

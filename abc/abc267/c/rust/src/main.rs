use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_m: usize,
        in_a: [i64; in_n],
    }

    let mut accum = vec![0; in_n + 1];
    for i in 0..in_n {
        accum[i + 1] = accum[i] + in_a[i];
    }

    let mut s = vec![0; in_n - in_m + 1];
    for i in 0..in_m {
        s[0] += (i + 1) as i64 * in_a[i];
    }
    for i in 0..(in_n - in_m) {
        s[i + 1] = in_m as i64 * in_a[i + in_m] + s[i] - (accum[i + in_m] - accum[i]);
    }

    let ans = s.iter().max().unwrap();
    println!("{}", ans);
}

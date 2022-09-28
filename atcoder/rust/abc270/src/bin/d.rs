use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_k: usize,
        in_a: [usize; in_k],
    }

    // dp[i] := i 個の石がある山にたいして先手が取れる石の最大値
    let mut dp = vec![0; in_n + 1];

    for i in 1..=in_n {
        for &a in &in_a {
            if i < a {
                continue;
            }
            dp[i] = dp[i].max(i - dp[i - a]);
        }
    }

    println!("{}", dp[in_n]);
}

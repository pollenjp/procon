use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_m: usize,
        in_a: [i64; in_n],
    }

    // dp[i][j] : i番目までを見て, j 番目まで選んだ時の最大値
    let mut dp = vec![vec![std::i64::MIN; in_m + 1]; in_n + 1];
    for i in 0..=in_n {
        dp[i][0] = 0;
    }

    for i in 1..=in_n {
        for j in 1..=in_m {
            // i番目を選ばない場合
            dp[i][j] = dp[i - 1][j];

            // i番目を選ぶ場合
            if dp[i - 1][j - 1] > std::i64::MIN {
                dp[i][j] = std::cmp::max(dp[i][j], dp[i - 1][j - 1] + j as i64 * in_a[i - 1]);
            }
        }
    }

    let ans = dp[in_n][in_m];
    println!("{}", ans);
}

use proconio::input;

static MOD_INT: usize = 1000000007;

fn main() {
    input! {
        in_n: usize,
        in_b: usize,
        in_k: usize,
        in_c: [usize; in_k],
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![0; in_b]; in_n + 1];

    dp[0][0] = 1;

    for i in 0..in_n {
        for rem in 0..in_b {
            // dbg!(i, rem);
            for c in &in_c {
                // 10
                let rem_next = (rem * 10 + *c) % in_b;
                dp[i + 1][rem_next] += dp[i][rem];
                dp[i + 1][rem_next] %= MOD_INT;
            }
        }
    }

    println!("{}", dp[in_n][0]);
}

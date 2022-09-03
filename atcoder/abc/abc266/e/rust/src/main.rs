use proconio::input;

static p: f64 = 3.5;

fn main() {
    input! {
        in_n: usize,
    }

    let mut accum = vec![0; 6 + 1];
    for i in 1..=6 {
        accum[i] = i + accum[i - 1];
    }

    let mut dp: Vec<f64> = vec![0.0; in_n];
    dp[0] = p;

    for i in 1..in_n {
        let ng = dp[i - 1] as usize;
        let x = (accum[6] - accum[ng]) as f64;
        let y = ng as f64 * dp[i - 1];

        dp[i] = (x + y) / 6.0;
    }

    println!("{:.7}", dp[in_n - 1]);
}

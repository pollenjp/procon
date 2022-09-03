use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_txa: [(usize, usize, i64); in_n],
    }

    let max_time = in_txa.last().unwrap().0;

    // 0 | 1 2 3 4 5 | 6
    // 0, 6 は計算単純化のためのダミー
    let mut dp: Vec<Vec<i64>> = vec![vec![-1; 7]; max_time + 1];
    dp[0][1] = 0; // 初期位置

    let mut t_current = 0;
    for &(t, x, a) in in_txa.iter() {
        loop {
            t_current += 1;
            for i in 1..=5 {
                let mut val = dp[t_current - 1][i - 1];
                val = std::cmp::max(val, dp[t_current - 1][i]);
                val = std::cmp::max(val, dp[t_current - 1][i + 1]);
                dp[t_current][i] = val;
            }
            if t_current == t {
                let val = dp[t_current][x + 1];
                if val >= 0 {
                    dp[t_current][x + 1] += a;
                }
                break;
            }
        }
    }

    let ans = dp[max_time].iter().max().unwrap();
    println!("{}", ans);
}

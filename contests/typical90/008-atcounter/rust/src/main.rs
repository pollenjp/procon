use proconio::input;
// use std::io::Write;

fn main() {
    input! {
        in_n: usize,
        in_s: String,
    };

    let target_s = "atcoder";
    let div_num = (10 as i64).pow(9) + 7;
    // dbg!(&div_num);

    // dp: in_s の j番目の文字までを取得した際に target_s の i番目の文字までの文字列を作成できるパターン数
    // size: (target_s.len() + 1, in_n + 1)
    // axis 0: target_s の i番目の文字までの文字列を作成
    // axis 1: in_s の j番目の文字までを取得した状態
    let mut dp: Vec<Vec<i64>> = vec![vec![0; in_n + 1]; target_s.len() + 1];

    // init
    for it_i in in_s.chars().enumerate() {
        let (i, c) = it_i;
        if c == target_s.chars().nth(0).unwrap() {
            dp[1][i + 1] = dp[1][i] + 1;
            dp[1][i + 1] %= div_num;
        } else {
            dp[1][i + 1] = dp[1][i];
            dp[1][i + 1] %= div_num;
        }
        // print!("{}", &dp[1][i + 1]);
    }
    // std::io::stdout().flush().unwrap();
    // println!("");

    // dp
    for it_i in target_s.chars().enumerate() {
        let (i, target_c) = it_i;
        if i > 0 {
            for it_j in in_s.chars().enumerate() {
                let (j, in_c) = it_j;
                if target_c == in_c {
                    dp[i + 1][j + 1] = dp[i + 1][j] + dp[i][j];
                    dp[i + 1][j + 1] %= div_num;
                } else {
                    dp[i + 1][j + 1] = dp[i + 1][j];
                }
                // print!("{}", &dp[i + 1][j + 1]);
            }
            // std::io::stdout().flush().unwrap();
            // println!("");
        }
    }

    println!("{}", dp[target_s.len()][in_s.len()]);
}

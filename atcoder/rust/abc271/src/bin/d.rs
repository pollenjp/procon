use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_s: usize,
        in_ab: [(usize, usize); in_n],
    }

    // s[i][j] := i番目まで選択した時にの和がj
    // -1 : no
    // 0 : H
    // 1 : T
    let mut dp = vec![vec![-1; in_s + 1]; in_n + 1];
    dp[0][0] = 0;

    for i in 0..in_n {
        let x = in_ab[i].0;
        for j in 0..=in_s {
            if j < x {
                continue;
            }
            if dp[i][j - x] == -1 {
                continue;
            }
            dp[i + 1][j] = 0;
        }
        let x = in_ab[i].1;
        for j in 0..in_s {
            if j < x {
                continue;
            }
            if dp[i][j - x] == -1 {
                continue;
            }
            dp[i + 1][j] = 1;
        }
    }

    if dp[in_n][in_s] == -1 {
        println!("No");
        return;
    } else {
        println!("Yes");
    }

    let mut ans_rev = vec![];
    let mut k = in_s as i64;
    for i in (0..in_n).rev() {
        if dp[i + 1][k as usize] == 0 {
            ans_rev.push('H');
            k -= in_ab[i].0 as i64;
        } else {
            ans_rev.push('T');
            k -= in_ab[i].1 as i64;
        }
    }

    println!("{}", ans_rev.iter().rev().collect::<String>());
}

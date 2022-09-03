use proconio::input;

static MAX_D: usize = 5_000;

fn print_table<T>(table: &Vec<Vec<T>>)
where
    T: std::fmt::Display,
{
    let row_size = table.len();
    if row_size == 0 {
        return;
    }
    let col_size = table[0].len();
    let mut col_size_vec = vec![0; col_size];
    for row in table.iter() {
        for (c_idx, cell) in row.iter().enumerate() {
            let s = format!("{}", cell);
            col_size_vec[c_idx] = std::cmp::max(col_size_vec[c_idx], s.len());
        }
    }
    for row in table.iter() {
        for (c_idx, cell) in row.iter().enumerate() {
            print!("{:>width$} ", cell, width = col_size_vec[c_idx] + 1);
        }
        println!();
    }
}

fn main() {
    input! {
        in_n: usize,
        in_dcs: [(usize, usize, usize); in_n]
    }

    // dbg!(&in_n);
    // dbg!(&in_dcs);

    // 締切が速い順
    let mut sorted_dcs = in_dcs;
    sorted_dcs.sort_by(|a, b| a.0.cmp(&b.0));

    // p[i][j]: i番目までの仕事を見て, j日目までに獲得できる報酬が最大となる場合の最大報酬
    let mut dp = vec![vec![0; MAX_D + 1]; in_n + 1];

    for n_idx in 1..in_n + 1 {
        for d_idx in 1..MAX_D + 1 {
            let &(d, c, s) = &sorted_dcs[n_idx - 1];
            let mut max_p = dp[n_idx][d_idx];

            // 現在の仕事 n_idx を実施しない場合
            max_p = std::cmp::max(max_p, dp[n_idx - 1][d_idx]);

            if d_idx > d {
                // 現在の仕事 n_idx の締切が過ぎている場合
                max_p = std::cmp::max(max_p, dp[n_idx][d_idx - 1]);
            } else if d_idx >= c {
                // 現在の仕事 n_idx を行うことができる場合
                max_p = std::cmp::max(max_p, dp[n_idx - 1][d_idx - c] + s);
            }

            dp[n_idx][d_idx] = max_p;
        }
    }
    // print_table(&dp);

    println!("{}", dp[in_n][MAX_D]);
}

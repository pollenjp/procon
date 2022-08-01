use proconio::input;
use std::io::stdout;
use std::io::Write;

static P: usize = 998244353;

fn print_array_dim3<T>(table: &Vec<Vec<Vec<T>>>)
where
    T: std::fmt::Display,
{
    let row_size = table.len();
    if row_size == 0 {
        return;
    }
    let col_size = table.iter().map(|row| row.len()).max().unwrap_or(0);

    let col_size_for_index = format!("{}", table.len()).len();
    let mut col_size_vec = vec![0; col_size];
    let mut k_row_size_per_row = vec![0; table.len()];
    for row_idx in 0..table.len() {
        for col_idx in 0..table[row_idx].len() {
            let k_len = table[row_idx][col_idx].len();
            for k in 0..k_len {
                col_size_vec[col_idx] = std::cmp::max(
                    col_size_vec[col_idx],
                    format!("{}", &table[row_idx][col_idx][k]).len(),
                );
            }
            k_row_size_per_row[row_idx] = std::cmp::max(k_row_size_per_row[row_idx], k_len);
        }
    }

    let colum_delimiter = "|";
    let colum_interval = colum_delimiter.len();
    let mut width = col_size_for_index + 1;
    width += (0..col_size)
        .map(|i| col_size_vec[i] + colum_interval)
        .sum::<usize>();
    let line_separator = "-".to_string().repeat(width);

    // print col index number
    print!("{:>width$}", " ", width = col_size_for_index);
    print!("{}", colum_delimiter);
    for col_idx in 0..col_size {
        print!("{:>width$}", col_idx, width = col_size_vec[col_idx]);
        print!("{}", colum_delimiter);
    }
    stdout().flush().unwrap();
    println!();

    println!("{}", line_separator);

    for row_idx in 0..table.len() {
        let k_row_size = k_row_size_per_row[row_idx];
        for k_row_idx in 0..k_row_size {
            // print index number
            if k_row_idx == 0 {
                print!("{:>width$}", row_idx, width = col_size_for_index);
            } else {
                print!("{:>width$}", " ", width = col_size_for_index);
            }
            print!("{}", colum_delimiter);

            for col_idx in 0..table[row_idx].len() {
                let k_len = table[row_idx][col_idx].len();
                if k_row_idx < k_len {
                    let val = &table[row_idx][col_idx][k_row_idx];
                    print!("{:>width$}", val, width = col_size_vec[col_idx]);
                } else {
                    print!("{:>width$}", " ", width = col_size_vec[col_idx]);
                }
                print!("{}", colum_delimiter);
            }
            stdout().flush().unwrap();
            println!();
        }
        println!("{}", line_separator);
    }
}

fn main() {
    input! {
        in_n: usize,
        in_a: [usize; in_n],
    }

    // let in_a = in_a.iter().map(|&a| a % in_n).collect::<Vec<usize>>();

    // for k in 0..in_n {
    //     dp[0][0][k] = 1;
    // }

    let mut ans = 0;
    for i in 1..=in_n {
        // dp[j][k][l] := 先頭から j 番目までを見て k 個 (k <= j) 選んだ際に剰余が l になるような選び方 (mod p)
        let mut dp = vec![vec![vec![0; in_n]; in_n + 1]; in_n + 1];

        let a_vec = in_a.iter().map(|&a| a % i).collect::<Vec<usize>>();
        for j in 1..=in_n {
            for k in 1..=std::cmp::min(i, j) {
                let a = a_vec[j - 1];
                // println!("{} {} {} {}", i, j, k, a);
                if k == 1 {
                    dp[j][k][a] += 1;
                }
                for l in 0..i {
                    let l_pre = (i + l - a) % i;
                    dp[j][k][l] += dp[j - 1][k - 1][l_pre];
                    dp[j][k][l] %= P;

                    dp[j][k][l] += dp[j - 1][k][l];
                    dp[j][k][l] %= P;
                }
            }
        }
        // print_array_dim3(&dp);
        ans += dp[in_n][i][0];
        ans %= P;
    }

    println!("{}", ans);
}

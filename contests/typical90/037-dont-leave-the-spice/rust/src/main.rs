use proconio::input;
use std::collections::HashSet;

fn print_table(table: &Vec<Vec<usize>>) {
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
    for row in table.iter().rev() {
        for (c_idx, cell) in row.iter().enumerate() {
            print!("{:>width$} ", cell, width = col_size_vec[c_idx] + 1);
        }
        println!();
    }
}

fn main() {
    input! {
        in_w: usize,
        in_n: usize,
        in_lrv: [(usize, usize, usize); in_n],
    };

    // 座標圧縮
    let mut st = HashSet::new();
    for &(l, r, _) in &in_lrv {
        st.insert(l);
        st.insert(r);
    }
    st.insert(in_w);
    let mut vals: Vec<usize> = st.into_iter().collect();
    vals.sort();

    let in_lrv = in_lrv
        .iter()
        .map(|(l, r, v)| {
            (
                vals.binary_search(&*l).unwrap(),
                vals.binary_search(&*r).unwrap(),
                v,
            )
        })
        .collect::<Vec<_>>();
    let in_w = vals.binary_search(&in_w).unwrap();

    // i (ax0-index) 番目の対象物を選択する際に重みが合計でちょうど j (ax1-index) になるときの最大価値
    let mut dp = vec![vec![0; in_w + 1]; in_n + 1];
    for i in 1..in_n + 1 {
        let (l, r, v) = in_lrv[i - 1];

        for j in 1..=in_w {
            // case: not select i-th item
            dp[i][j] = std::cmp::max(dp[i][j], dp[i - 1][j]);
            // case: select i-th item
            let (k_l, k_r) = (
                std::cmp::max(j as i32 - l as i32, 0) as usize,
                std::cmp::max(j as i32 - r as i32, in_w as i32) as usize,
            );
            for k in k_l..=k_r {
                dp[i][j] = std::cmp::max(dp[i][j], dp[i - 1][k] + v);
            }
        }
    }

    print_table(&dp);

    let v = dp[in_n][in_w];
    if v > 0 {
        println!("{}", v);
    } else {
        println!("-1");
    }
}

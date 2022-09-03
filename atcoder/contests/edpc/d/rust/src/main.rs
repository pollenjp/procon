fn main() {
    proconio::input! {
        in_n: usize,
        in_w: usize,
        in_wv_vec: [[i64; 2]; in_n],
    }

    let mut dp_tensor2 = vec![vec![0 as i64; in_w + 1]; in_n + 1];

    for i in 0..in_n {
        // i番目の選択
        let w = in_wv_vec[i][0] as usize;
        let v = in_wv_vec[i][1];
        for j in 0..=in_w {
            if j as i64 - w as i64 >= 0 {
                dp_tensor2[i + 1][j] = std::cmp::max(dp_tensor2[i][j - w] + v, dp_tensor2[i][j]);
            } else {
                dp_tensor2[i + 1][j] = dp_tensor2[i][j];
            }
        }
    }

    println!("{}", dp_tensor2[in_n].iter().max().unwrap());
}

fn main() {
    let num_actions = 3;
    proconio::input! {
        in_n: usize,
        in_abc_vec: [[i32; num_actions]; in_n],
    }

    // i (0 origin) 番目に j の行動をとった場合の最小コスト
    let mut dp_tensor2 = vec![vec![0; num_actions]; in_n + 1];

    for i in 0..in_n {
        for j in 0..num_actions {
            dp_tensor2[i + 1][j] = std::cmp::max(
                dp_tensor2[i][(j + 1) % num_actions] + in_abc_vec[i][(j + 1) % num_actions],
                dp_tensor2[i][(j + 2) % num_actions] + in_abc_vec[i][(j + 2) % num_actions],
            );
        }
    }

    println!("{}", dp_tensor2[in_n].iter().max().unwrap());
}

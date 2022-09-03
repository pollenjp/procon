fn main() {
    proconio::input! {
        in_n: usize,
        in_k: usize,
        in_h_vec: [i32 ; in_n],
    }

    // cost
    let max_val = 1000000001;
    let mut dp_tensor = vec![max_val; in_n];
    dp_tensor[0] = 0;

    for i in 0..in_n {
        // i 番目の足場を通った場合でその位置でのの最小コスト
        for j in 1..std::cmp::min(i, in_k) + 1 {
            dp_tensor[i] = std::cmp::min(
                dp_tensor[i],
                dp_tensor[i - j] + (in_h_vec[i - j] - in_h_vec[i]).abs(),
            );
        }
    }

    println!("{}", dp_tensor[in_n - 1]);
}

fn main() {
    proconio::input! {
        in_n: usize,
        in_h_vec: [i32 ; in_n],
    }

    // cost
    let mut dp_tensor = vec![0; in_n];

    for i in 0..in_n {
        // i 番目の足場を通った場合でその位置でのの最小コスト
        if i == 0 {
            dp_tensor[i] = 0;
            continue;
        } else if i == 1 {
            dp_tensor[i] = (in_h_vec[i] - in_h_vec[i - 1]).abs();
            continue;
        }
        dp_tensor[i] = std::cmp::min(
            dp_tensor[i - 1] + (in_h_vec[i] - in_h_vec[i - 1]).abs(),
            dp_tensor[i - 2] + (in_h_vec[i] - in_h_vec[i - 2]).abs(),
        );
    }

    println!("{}", dp_tensor[in_n - 1]);
}

use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_a: i64,
        in_b: i64,
        in_c: i64,
    }

    let coin_num_max = 9999;

    let mut coin_num_min = std::i64::MAX;

    let a_max_num = std::cmp::min(coin_num_max, in_n as i64 / in_a);
    for a_num in 0..=a_max_num {
        let b_max_num = std::cmp::min(coin_num_max, (in_n as i64 - a_num * in_a) / in_b);
        for b_num in 0..=b_max_num {
            let left = in_n as i64 - a_num * in_a - b_num * in_b;
            if left % in_c == 0 {
                let c_num = left / in_c;
                coin_num_min = std::cmp::min(coin_num_min, a_num + b_num + c_num);
            }
        }
    }

    println!("{}", coin_num_min);
}

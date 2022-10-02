use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_a: [usize; in_n],
    }

    let mut current_xor: usize = 0;
    let mut cnt: usize = 0;
    let mut r = 0;
    for l in 0..in_n {
        loop {
            if current_xor & in_a[r] != 0 {
                break;
            }
            cnt += 1;
            if r + 1 >= in_n {
                // 最後に index 以上を指していた場合
                break;
            }
            cnt += r - l;
            current_xor ^= in_a[r];
            r += 1;
        }

        // l を増やす前に引いておく
        // ２回の xor 操作は 0 になる
        current_xor ^= in_a[l];
    }

    println!("{}", cnt);
}

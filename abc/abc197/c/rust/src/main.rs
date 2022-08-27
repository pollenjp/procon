use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_a_vec: [usize; in_n],
    }

    let mut ans = std::usize::MAX;

    let mut i = 0;
    while i < 1 << (in_n - 1) {
        let mut xor_val = 0;
        let mut or_val = 0;

        // 1 <= j < n : each 1-bit of binary j is a splitter.
        // j == n : All bit is zero (no splitters).
        for j in 0..=in_n {
            if j < in_n {
                or_val |= in_a_vec[j];
            }
            if j == in_n
            // if right end
            || i >> j & 1 == 1
            // if found a splitter
            {
                xor_val ^= or_val;
                or_val = 0;
            }
        }

        ans = std::cmp::min(ans, xor_val);
        i += 1;
    }

    println!("{}", ans);
}

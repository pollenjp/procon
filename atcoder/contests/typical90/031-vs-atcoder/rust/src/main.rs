use std::vec;

use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_w: [usize; in_n],
        in_b: [usize; in_n],
    }
    const MAX_INIT_W: usize = 50;
    const MAX_INIT_B: usize = 50;
    const MAX_B: usize = MAX_INIT_B + MAX_INIT_W * (MAX_INIT_W + 1) / 2; // 等比数列の和

    // ax1 は b_i + w_i の max が確保できればOK
    let mut grundy = vec![vec![0; MAX_B + 1 + MAX_INIT_W + 1]; MAX_INIT_W + 1];

    for w in 0..MAX_INIT_W {
        grundy[w][0] = 1;
    }

    for w_i in 0..=MAX_INIT_W {
        for b_i in 0..=MAX_B {
            let mut mex = [0; MAX_INIT_W + 1 + MAX_B + 1];
            if w_i >= 1 {
                mex[grundy[w_i - 1][b_i + w_i]] = 1;
            }

            if b_i >= 2 {
                for k in 1..=(b_i / 2) {
                    mex[grundy[w_i][b_i - k]] = 1;
                }
            }

            for k in 0..mex.len() {
                if mex[k] == 0 {
                    grundy[w_i][b_i] = k;
                    break;
                }
            }
        }
    }

    let mut sum_xor = 0;
    for i in 0..in_n {
        sum_xor ^= grundy[in_w[i]][in_b[i]];
    }

    if sum_xor != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}

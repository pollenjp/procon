use proconio::input;

// y の上位集合でかつ, x より大きく, x に最も近い値を返す
fn solve(x: usize, mut y: usize) -> usize {
    for i in (0..32 as usize).rev() {
        if (y >> i) & 1 != 0 {
            continue;
        }

        // y で 0 bit目から i - 1 bit目であらわされる数が x 未満である場合
        if (y | (1 << i) - 1) < x {
            // i bit目を反転させる
            y ^= 1 << i;
        }
    }
    y
}

fn main() {
    input! {
        in_n: usize,
        in_m: usize,
        in_k: usize,
        in_a: [usize; in_n],
    }

    let mut ans = 0;

    // i bit目
    for i in (0..32 as usize).rev() {
        let mut b = vec![0; in_n];
        for j in 0..in_n {
            b[j] = solve(in_a[j], ans | (1 << i)) - in_a[j];
        }

        b.sort();

        let m = b[0..in_k].iter().sum::<usize>();
        if m <= in_m {
            ans |= 1 << i;
        }
    }
    println!("{}", ans);
}

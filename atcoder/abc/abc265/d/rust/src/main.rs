use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_p: usize,
        in_q: usize,
        in_r: usize,
        in_a: [usize; in_n],
    }

    // index 0 is 0
    let mut accum = vec![0; in_n + 1];
    accum[0] = in_a[0];

    for i in 0..in_n {
        accum[i + 1] = accum[i] + in_a[i];
    }

    // 尺取り法

    // 以下では x, y, z, w は問題文より後ろに1つずれている

    // p

    let mut x = 0;
    let mut y = x + 1;
    let mut p_current = accum[y] - accum[x];
    while p_current != in_p {
        if p_current < in_p {
            y += 1;
            if y > in_n - 2 {
                print!("No");
                return;
            }
        } else {
            x += 1;
        }
        p_current = accum[y] - accum[x];
    }

    // q

    let mut z = y + 1;
    let mut q_current = accum[z] - accum[y];
    while q_current != in_q {
        if q_current < in_q {
            z += 1;
            if z > in_n - 1 {
                print!("No");
                return;
            }
        } else {
            y += 1;
        }
        q_current = accum[z] - accum[y];
    }

    // r

    let mut w = z + 1;
    let mut r_current = accum[w] - accum[z];
    while r_current != in_r {
        if r_current < in_r {
            w += 1;
            if w > in_n {
                print!("No");
                return;
            }
        } else {
            z += 1;
        }
        r_current = accum[w] - accum[z];
    }

    print!("Yes");
}

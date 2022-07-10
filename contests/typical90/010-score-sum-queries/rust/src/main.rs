use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_cp: [(usize, i32); in_n],
        in_q: usize,
        in_lr: [(usize, usize); in_q],
    };

    // accumulated sum
    let mut accum1 = vec![0; in_n + 1];
    let mut accum2 = vec![0; in_n + 1];
    for it_i in in_cp.iter().enumerate() {
        let (i, (class, point)) = it_i;

        if *class == 1 {
            accum1[i + 1] = accum1[i] + *point;
            accum2[i + 1] = accum2[i];
        } else {
            // class == 2
            accum1[i + 1] = accum1[i];
            accum2[i + 1] = accum2[i] + *point;
        }
    }

    for it_i in in_lr.iter().enumerate() {
        let (_, (l, r)) = it_i;
        println!(
            "{} {}",
            accum1[*r] - accum1[*l - 1],
            accum2[*r] - accum2[*l - 1]
        );
    }
}

use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_q: usize,
    }

    let mut table = vec![];
    for _ in 0..in_n {
        input! {
            in_l: usize,
            in_a: [usize; in_l],
        };
        table.push(in_a);
    }

    input! {
        in_st: [(usize, usize); in_q],
    }
    let st = in_st
        .iter()
        .map(|(s, t)| (s - 1, t - 1))
        .collect::<Vec<_>>();

    for (i, j) in st {
        println!("{}", table[i][j]);
    }
}

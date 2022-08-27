use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_a: [usize; in_n],
    }

    let mut a = in_a;
    a.sort();
    let x = a.into_iter().rev().collect::<Vec<_>>()[..3].to_vec();
    let mut ys = vec![];
    for v in (0..3).permutations(3) {
        let y = format!("{}{}{}", x[v[0]], x[v[1]], x[v[2]])
            .parse::<usize>()
            .unwrap();
        ys.push(y);
    }
    println!("{}", ys.iter().max().unwrap());
}

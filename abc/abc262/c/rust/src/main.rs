use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_a: [usize; in_n],
    }

    let in_a = in_a.iter().map(|&a| a - 1).collect::<Vec<usize>>();

    // 自身のindex

    let mut n1: usize = 0;
    for (i, a) in in_a.iter().enumerate() {
        if *a == i {
            n1 += 1;
        }
    }

    // pair 相手の index
    let mut pair_list = vec![std::usize::MAX; in_n];
    let mut n2: usize = 0;
    for (i, a) in in_a.iter().enumerate() {
        if *a > i {
            pair_list[i] = *a;
        } else if *a < i {
            pair_list[i] = *a;
            if pair_list[*a] == i {
                n2 += 1;
            }
        }
    }

    if n1 == 0 {
        // 1でも結果は変わらない
        n1 = 1;
    }

    println!("{}", n1 * (n1 - 1) / 2 + n2);
}

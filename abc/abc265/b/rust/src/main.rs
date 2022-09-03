use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_m: usize,
        in_t: usize,
        in_a: [usize; in_n-1],
        in_xy: [(usize, usize); in_m],
    }

    let in_xy = in_xy.iter().map(|&(x, y)| (x - 1, y)).collect::<Vec<_>>();

    let mut bonus = vec![0; in_n];
    for &(x, y) in &in_xy {
        bonus[x] = y;
    }

    let mut your_time = in_t;

    for i in 0..in_n - 1 {
        // dbg!(your_time);
        // i -> i+1
        if your_time <= in_a[i] {
            println!("No");
            return;
        }
        your_time -= in_a[i];

        if i == in_n - 1 {
            // 最後の部屋
            break;
        }
        your_time += bonus[i + 1];
    }

    // dbg!(your_time);

    println!("Yes");
}

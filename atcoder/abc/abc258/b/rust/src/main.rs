// modulo i64
// return one of the 0..=m-1
fn modi(n: i64, m: i64) -> usize {
    (((n % m) + m) % m) as usize
}

fn main() {
    proconio::input! {
        in_n: usize,
        in_a: [String; in_n]
    }

    let table_a = in_a
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let coord_offset = [
        (0, -1),  // up
        (1, -1),  // up-right
        (1, 0),   // right
        (1, 1),   // down-right
        (0, 1),   // down
        (-1, 1),  // down-left
        (-1, 0),  // left
        (-1, -1), // up-left
    ];
    let mut max_num = 0;
    for i in 0..in_n {
        for j in 0..in_n {
            let mut r = i;
            let mut c = j;
            for &(r_off, c_off) in &coord_offset {
                let mut now = 0;
                for _ in 0..in_n {
                    now *= 10;
                    now += table_a[r as usize][c];

                    r = modi(r as i64 + r_off, in_n as i64);
                    c = modi(c as i64 + c_off, in_n as i64);
                }
                max_num = std::cmp::max(max_num, now);
            }
        }
    }

    println!("{}", max_num);
}

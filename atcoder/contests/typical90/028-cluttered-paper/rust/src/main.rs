use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_lr: [(usize, usize, usize, usize); in_n],
    }
    let x_max = in_lr.iter().map(|&(_, _, x, _)| x).max().unwrap();
    let y_max = in_lr.iter().map(|&(_, _, _, y)| y).max().unwrap();

    let mut table = vec![vec![0; x_max + 2]; y_max + 2];
    // いもす法
    for &(lx, ly, rx, ry) in &in_lr {
        table[ly][lx] += 1;
        table[ry][rx] += 1;
        table[ly][rx] -= 1;
        table[ry][lx] -= 1;
    }

    // accumulate
    let row_size = table.len();
    if row_size == 0 {
        // noop
        return;
    }
    let col_size = table[0].len();
    for row in table.iter_mut() {
        for col in 1..row.len() {
            row[col] += row[col - 1];
        }
    }
    for col_idx in 0..col_size {
        for row_idx in 1..row_size {
            table[row_idx][col_idx] += table[row_idx - 1][col_idx];
        }
    }

    let mut cnt = vec![0; in_n + 1];
    for row in table.iter() {
        for cell in row.iter() {
            cnt[*cell as usize] += 1;
        }
    }

    for c in cnt.drain(1..) {
        println!("{}", c);
    }
}

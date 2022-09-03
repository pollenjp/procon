use proconio::input;

fn walk(
    row: usize,
    col: usize,
    g: &Vec<Vec<char>>,
    way: &mut Vec<Vec<bool>>,
) -> Option<(usize, usize)> {
    if way[row][col] {
        return None;
    }
    way[row][col] = true;
    match g[row][col] {
        'U' => {
            if row == 0 {
                return Some((row, col));
            }

            return walk(row - 1, col, g, way);
        }
        'D' => {
            if row == g.len() - 1 {
                return Some((row, col));
            }

            return walk(row + 1, col, g, way);
        }
        'L' => {
            if col == 0 {
                return Some((row, col));
            }

            return walk(row, col - 1, g, way);
        }
        'R' => {
            if col == g[0].len() - 1 {
                return Some((row, col));
            }

            return walk(row, col + 1, g, way);
        }
        _ => {
            panic!("invalid input");
        }
    }
}

fn main() {
    input! {
        in_h: usize,
        in_w: usize,
        in_g: [String; in_h],
    }

    let in_g = in_g.iter().map(|s| s.chars().collect()).collect();

    let mut way = vec![vec![false; in_w]; in_h];
    match walk(0, 0, &in_g, &mut way) {
        Some((row, col)) => {
            println!("{} {}", row + 1, col + 1);
        }
        None => {
            println!("{}", -1);
        }
    }
}

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::time::Instant;

// modulo i32
// return one of the 0..=m-1
fn modi(n: i32, m: i32) -> i32 {
    ((n % m) + m) % m
}

fn main() {
    proconio::input! {
        in_n: usize,
        in_a: [String; in_n]
    }

    let table_a: Vec<Vec<i32>> = in_a
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let mut deq = VecDeque::new();

    let max_val = table_a
        .iter()
        .map(|x| x.iter().max().unwrap())
        .max()
        .unwrap();

    for i in 0..in_n {
        for j in 0..in_n {
            if table_a[i][j] == *max_val {
                deq.push_back((i, j)); // (i,j) , k is left
            }
        }
    }

    fn dfs(
        table_a: &Vec<Vec<i32>>,
        max_visit: usize,
        coord: (i32, i32),
        visited: &mut HashSet<(i32, i32)>,
        current_num: i32,
    ) -> i32 {
        if visited.len() == max_visit {
            return current_num;
        }

        let current_num = current_num * 10 + table_a[coord.0 as usize][coord.1 as usize];
        let (row_num, col_num) = (table_a.len() as i32, table_a[0].len() as i32);

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

        type CoordMap = HashMap<(i32, i32), i32>;
        let mut coords: CoordMap = HashMap::new();
        for (r, c) in coord_offset {
            let next_coord = (modi(coord.0 + r, row_num), modi(coord.1 + c, col_num));
            if visited.contains(&next_coord) {
                continue;
            }
            let (r, c) = next_coord;
            coords.insert(next_coord, table_a[r as usize][c as usize]);
        }

        if coords.len() == 0 {
            return -1; // no more path
        }

        let max_val = coords.iter().map(|(_, v)| *v).max().unwrap();

        let mut max_ret = 0;
        let time = Instant::now();
        visited.insert(coord);
        for (&(r, c), &v) in coords.iter() {
            if v < max_val {
                continue;
            }
            let ret_s = dfs(table_a, max_visit, (r, c), visited, current_num);
            if ret_s > max_ret {
                max_ret = ret_s;
            }
        }
        visited.remove(&coord);

        if visited.len() == 2 {
            dbg!(time.elapsed(), max_ret);
        }

        return max_ret;
    }

    let mut max_num = 0;
    for (i, j) in deq {
        let current_num = dfs(
            &table_a,
            in_n,
            (i as i32, j as i32),
            &mut HashSet::from([(i as i32, j as i32)]),
            0,
        );
        if current_num > max_num {
            max_num = current_num;
        }
    }

    println!("{}", max_num);
}

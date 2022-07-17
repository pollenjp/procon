use std::collections::HashSet;
use std::collections::VecDeque;

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
        max_visit: i32,
        coord: (i32, i32),
        visited: &HashSet<(i32, i32)>,
        current_num: i32,
    ) -> i32 {
        if visited.len() == max_visit as usize {
            return current_num;
        }

        let current_num = current_num * 10 + table_a[coord.0 as usize][coord.1 as usize];
        let (row_num, col_num) = (table_a.len() as i32, table_a[0].len() as i32);
        let mut visited = visited.clone();
        visited.insert(coord);

        let coord_offset = vec![
            (0, -1),  // up
            (1, -1),  // up-right
            (1, 0),   // right
            (1, 1),   // down-right
            (0, 1),   // down
            (-1, 1),  // down-left
            (-1, 0),  // left
            (-1, -1), // up-left
        ];

        let mut coords = HashSet::new();
        for (r, c) in coord_offset {
            let next_coord = (modi(coord.0 + r, row_num), modi(coord.1 + c, col_num));
            if visited.contains(&next_coord) {
                continue;
            }
            coords.insert(next_coord);
        }

        let val_vec = coords
            .iter()
            .map(|&(x, y)| table_a[x as usize][y as usize])
            .collect::<Vec<i32>>();
        if val_vec.len() == 0 {
            return -1; // no more path
        }

        let coord_vec = coords.iter().collect::<Vec<_>>();
        let max_val = val_vec.iter().max().unwrap();

        let val_vec_with_idx = val_vec
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, v)| v == max_val)
            .collect::<Vec<(usize, i32)>>();

        let mut max_ret = 0;
        for (i, _) in val_vec_with_idx {
            let &(r, c) = coord_vec[i];
            let ret_s = dfs(table_a, max_visit, (r, c), &visited, current_num);
            if ret_s > max_ret {
                max_ret = ret_s;
            }
        }

        return max_ret;
    }

    let mut max_num = 0;
    for (i, j) in deq {
        let current_num = dfs(
            &table_a,
            in_n as i32,
            (i as i32, j as i32),
            &HashSet::new(),
            0,
        );
        if current_num > max_num {
            max_num = current_num;
        }
    }

    println!("{}", max_num);
}

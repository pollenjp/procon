use proconio::input;
fn follow_route(r: usize, c: usize, grid_connect: &Vec<Vec<(usize, usize)>>) -> (usize, usize) {
    let (mut r, mut c) = (r, c);
    while (r, c) != grid_connect[r][c] {
        (r, c) = grid_connect[r][c];
    }
    (r, c)
}

fn main() {
    input! {
        in_h: usize,
        in_w: usize,
        in_q: usize,
    }

    let mut grid_color = vec![vec![0; in_w]; in_h];

    // 自身のマスが塗られていた場合, 同じく塗られた接続関係にあるマスの中で, 最も row が小さいものの中で, 最も col が小さいものを格納する.
    // (row, col)
    let mut grid_connect = vec![vec![(0, 0); in_w]; in_h];
    for i in 0..grid_connect.len() {
        for j in 0..grid_connect[i].len() {
            grid_connect[i][j].0 = i;
            grid_connect[i][j].1 = j;
        }
    }

    for _ in 0..in_q {
        input! {
            in_q_label: usize,
        }

        match in_q_label {
            1 => {
                input! {
                    q_r: usize,
                    q_c: usize,
                }
                let (q_r, q_c) = (q_r - 1, q_c - 1);

                grid_color[q_r][q_c] = 1;
                // up, left, down, right
                let r_offsets = [-1, 0, 1, 0];
                let c_offsets = [0, -1, 0, 1];

                let mut candidates = vec![];

                for i in 0..r_offsets.len() {
                    let r = q_r as i32 + r_offsets[i];
                    let c = q_c as i32 + c_offsets[i];
                    if r < 0 || c < 0 || r >= in_w as i32 || c >= in_h as i32 {
                        continue;
                    }
                    let r = r as usize;
                    let c = c as usize;

                    if grid_color[r][c] != 1 {
                        continue;
                    }

                    candidates.push((r, c));
                }

                if candidates.is_empty() {
                    continue;
                }

                // update grid_connect partially (candidates)

                // calc min row, col
                let min_coord = *candidates.iter().min().unwrap();

                // update grid_connect
                grid_connect[q_r][q_c] = min_coord;
                for (r, c) in candidates.iter() {
                    grid_connect[*r][*c] = min_coord;
                    // grid_connect[*r][*c].0 = min_coord.0;
                    // grid_connect[*r][*c].1 = min_coord.1;
                }
            }
            2 => {
                input! {
                    q_ra: usize,
                    q_ca: usize,
                    q_rb: usize,
                    q_cb: usize,
                }
                let (q_ra, q_ca, q_rb, q_cb) = (q_ra - 1, q_ca - 1, q_rb - 1, q_cb - 1);

                // check both block is painted
                if grid_color[q_ra][q_ca] != 1 || grid_color[q_rb][q_cb] != 1 {
                    println!("No");
                    continue;
                }

                // check if the block is connected

                if dbg!(follow_route(q_ra, q_ca, &grid_connect))
                    == dbg!(follow_route(q_rb, q_cb, &grid_connect))
                {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => {
                panic!("error");
            }
        }
    }
}

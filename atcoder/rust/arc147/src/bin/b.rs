use proconio::input;

fn swap(i1: usize, i2: usize, p: &mut Vec<usize>, idx_arr: &mut Vec<usize>) {
    let tmp = p[i1];
    p[i1] = p[i2];
    p[i2] = tmp;
    idx_arr[p[i1]] = i1;
    idx_arr[p[i2]] = i2;
}

fn main() {
    input! {
        in_n: usize,
        in_p: [usize; in_n],
    };

    let mut p_arr = in_p.iter().map(|&p| p - 1).collect::<Vec<usize>>();
    let mut idx_arr = vec![0; in_n];
    for i in 0..in_n {
        idx_arr[p_arr[i]] = i;
    }

    let mut ans_arr = vec![];
    let mut cnt = 0;
    for i in 0..in_n {
        let mut idx_current = idx_arr[i];
        if idx_current % 2 != p_arr[idx_current] % 2 {
            // 偶奇を一致させる必要あり
            let start_idx = if idx_current % 2 == 0 { 1 } else { 0 };
            // 操作Bで隣になり得るものを探す
            for j in (start_idx..in_n).step_by(2) {
                if p_arr[j] % 2 != j % 2 {
                    // 偶奇が一致していないものを探す
                    let mut diff = j as i64 - idx_current as i64;
                    while diff.abs() > 1 {
                        let idx2 = if diff > 0 {
                            idx_current + 2
                        } else {
                            idx_current - 2
                        };
                        swap(idx2, idx_current, &mut p_arr, &mut idx_arr);
                        idx_current = idx2;
                        ans_arr.push(format!("B {}", idx_current + 1));
                        diff = j as i64 - idx_current as i64;
                        cnt += 1;
                    }

                    // A
                    let idx2 = if diff > 0 {
                        idx_current + 1
                    } else {
                        idx_current - 1
                    };
                    swap(idx2, idx_current, &mut p_arr, &mut idx_arr);
                    idx_current = idx2;
                    ans_arr.push(format!("A {}", idx_current + 1));
                    cnt += 1;
                    break;
                }
            }
        }

        while idx_current != i {
            if idx_current - i == 1 {
                // 操作A
                swap(idx_current - 1, idx_current, &mut p_arr, &mut idx_arr);
                idx_current -= 1;
                ans_arr.push(format!("A {}", idx_current + 1));
            } else {
                // 操作B
                swap(idx_current - 2, idx_current, &mut p_arr, &mut idx_arr);
                idx_current -= 2;
                ans_arr.push(format!("B {}", idx_current + 1));
            }
            cnt += 1;
            // dbg!(&p_arr, &idx_arr);
        }
    }
    // dbg!(&p_arr);
    // dbg!(&idx_arr);

    println!("{}", cnt);
    for ans in ans_arr {
        println!("{}", ans);
    }
}

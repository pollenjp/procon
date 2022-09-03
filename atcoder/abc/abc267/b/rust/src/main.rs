use proconio::input;

// 列
// 左から 0, 1, ..., 6
static COLS: [usize; 10] = [3, 2, 4, 1, 3, 5, 0, 2, 4, 6];

fn main() {
    input! {
        in_s: String,
    }

    if in_s.chars().nth(0).unwrap() == '1' {
        println!("No");
        return;
    }

    // 立っているピンの数
    let mut pin_nums: [usize; 7] = [1, 1, 2, 2, 2, 1, 1];
    for (i, flag) in in_s.chars().enumerate() {
        let col = COLS[i];
        if flag == '0' {
            pin_nums[col] -= 1;
        }
    }

    for i in 0..6 {
        for j in (i + 1)..7 {
            if pin_nums[i] > 0 && pin_nums[j] > 0 {
                for k in (i + 1)..j {
                    if pin_nums[k] == 0 {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No");
}

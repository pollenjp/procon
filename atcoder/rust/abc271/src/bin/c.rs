use proconio::input;

fn main() {
    input! {
        in_n: usize,
        in_a: [usize; in_n],
    }

    let mut in_a = in_a;

    in_a.sort();
    let mut expected = 0;
    let mut left_num = in_n;
    let mut cnt = 0;
    let mut i = 0;
    loop {
        expected += 1;

        if i >= left_num {
            break;
        }

        if i < in_a.len() {
            if in_a[i] == expected {
                i += 1;
                cnt += 1;
            } else if in_a[i] < expected {
                i += 1;
                left_num += 1;
                expected -= 1;
            } else if left_num < i + 2 {
                break;
            } else {
                left_num -= 2;
                cnt += 1;
            }
            continue;
        }

        if left_num < i + 2 {
            break;
        } else {
            left_num -= 2;
            cnt += 1;
        }
    }

    println!("{}", cnt);
}

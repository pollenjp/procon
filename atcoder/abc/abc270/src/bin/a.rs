use proconio::input;
fn main() {
    input! {
        in_a: usize,
        in_b: usize,
    }

    let mut a_solved = vec![false; 3];
    let mut b_solved = vec![false; 3];
    let mut c_solved = vec![false; 3];
    for i in 0..3 {
        if (in_a >> i) & 1 == 1 {
            a_solved[i] = true;
        }
        if (in_b >> i) & 1 == 1 {
            b_solved[i] = true;
        }

        if a_solved[i] || b_solved[i] {
            c_solved[i] = true;
        }
    }

    let mut c_point = 0;
    for i in 0..3 {
        if c_solved[i] {
            c_point += 1 << i;
        }
    }
    println!("{}", c_point);
}

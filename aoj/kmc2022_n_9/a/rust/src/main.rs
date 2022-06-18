fn main() {
    loop {
        // proconio::input! {
        input! {
            in_n: usize,
            in_m: usize,
        }
        if in_n == 0 && in_m == 0 {
            break;
        }
        // proconio::input! {
        input! {
            in_a_vec: [i32; in_n],
        }


        let mut best_sum: i32 = 0;

        for i in 0..in_n {
            for j in i+1..in_n {
                let sum = in_a_vec[i] + in_a_vec[j];
                if best_sum < sum && sum <= in_m as i32 {
                    best_sum = sum;
                }
            }
        }

        if best_sum == 0 {
            println!("NONE");
        } else {
            println!("{}", best_sum);
        }
    }
}

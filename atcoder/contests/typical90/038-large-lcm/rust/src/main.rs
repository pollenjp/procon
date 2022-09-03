use proconio::input;

fn euclidian_algorithm(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    euclidian_algorithm(b, a % b)
}

static MAX_VAL: usize = 1000000000000000000;

fn main() {
    input! {
        in_a: usize,
        in_b: usize,
    };

    let gcd = euclidian_algorithm(in_a, in_b);
    let a2 = in_a / gcd;
    let b2 = in_b / gcd;

    if MAX_VAL / a2 < b2 {
        println!("Large");
        return;
    }
    let ab2 = a2 * b2;

    if MAX_VAL / ab2 < gcd {
        println!("Large");
        return;
    }

    println!("{}", ab2 * gcd);
}

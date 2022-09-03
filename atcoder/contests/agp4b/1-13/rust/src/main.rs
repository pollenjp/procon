fn main() {
    proconio::input! {
        n: u32,
        v: [i32; n],
    }

    dbg!(n);

    for val in &v {
        dbg!(val);
    }

    let average = v.iter().fold(0, |sum, a| sum + a) / n as i32;
    dbg!(average);

    for val in &v {
        // println!("{}", val.abs_diff(average)); // Rust v1.60.0
        println!("{}", (val - average).abs());
    }
}

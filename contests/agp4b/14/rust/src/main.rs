fn main() {
    proconio::input! {
        abc: [i32; 3],
    }

    dbg!(abc.iter().min().unwrap(), abc.iter().max().unwrap());
    println!("{}", abc.iter().max().unwrap() - abc.iter().min().unwrap())
}

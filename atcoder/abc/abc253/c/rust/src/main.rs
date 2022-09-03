use std::collections::BTreeSet;

use std::io::BufRead;

fn main() {
    proconio::input! {
        in_q: usize,
    }

    let max_x = 1000000000;
    let mut vec: Vec<i32> = vec![0; max_x];

    let mut set = BTreeSet::new();

    for _ in 0..in_q {
        let mut query = String::new();
        std::io::stdin()
            .lock()
            .read_line(&mut query)
            .expect("query string");

        // dbg!(query.trim().to_string().split(" ").collect::<Vec<&str>>());
        let query_vec: Vec<&str> = query.trim().split(" ").collect::<Vec<&str>>();
        match &query_vec[..] {
            [_, x] => {
                let x = x.parse::<i32>().unwrap();
                if vec[x as usize] == 0 {
                    set.insert(x);
                }
                vec[x as usize] += 1;
                // dbg!("Hello");
            }
            [_, x, c] => {
                let x = x.parse::<i32>().unwrap();
                let c = c.parse::<i32>().unwrap();
                let minus_val = c.min(vec[x as usize]);
                vec[x as usize] -= minus_val;
                if vec[x as usize] == 0 {
                    set.remove(&x);
                }
            }
            [_] => {
                let diff = set.last().unwrap() - set.first().unwrap();
                // dbg!(set.map.first_key_value().unwrap().1);
                println!("{}", diff);
            }
            _ => {}
        }

        // println!("{}", ans);
    }
}

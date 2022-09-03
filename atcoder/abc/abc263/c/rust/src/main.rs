use proconio::input;

struct Num {
    n_vec: Vec<usize>, // 0, 1, 2, 3, ..., n 桁目
    m: usize,          // 各桁の最大値
}

impl Num {
    fn incr(&mut self) -> bool {
        let mut i = 0;
        loop {
            if self.n_vec[i] < self.m {
                self.n_vec[i] += 1;
                let mut flag = true;
                for j in 0..i {
                    let j_rev = i - j - 1;
                    self.n_vec[j_rev] = self.n_vec[i] + (j + 1);
                    if self.n_vec[j_rev] > self.m {
                        flag = false;
                        break;
                    }
                }
                if flag {
                    return true;
                }
            }

            i += 1;
            if i == self.n_vec.len() {
                return false;
            }
        }
    }
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.n_vec
                .iter()
                .rev()
                .fold(String::new(), |acc, x| acc + &format!("{} ", &x))
        )
    }
}

fn main() {
    input! {
        in_n: usize,
        in_m: usize,
    }

    let mut n_vec = vec![0; in_n];
    for i in 0..in_n {
        n_vec[i] = in_n - i;
    }

    let mut num = Num {
        n_vec: n_vec,
        m: in_m,
    };

    println!("{}", num);
    loop {
        if num.incr() {
            println!("{}", num);
        } else {
            break;
        }
    }
}

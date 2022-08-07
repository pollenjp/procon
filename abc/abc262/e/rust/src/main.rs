use proconio::input;
use std::option::Option;

static MOD_INT: usize = 998244353;

struct FactorialMod<T>
where
    T: Copy
        + Clone
        + Default
        + num::PrimInt
        + num::Zero
        + num::One
        + num::NumCast
        + num::Signed
        + std::fmt::Debug
        + std::ops::Add
        + std::ops::AddAssign
        + std::ops::Sub
        + std::ops::SubAssign
        + std::ops::Mul
        + std::ops::MulAssign
        + std::ops::Neg
        + std::ops::Rem
        + std::ops::RemAssign,
{
    fact: Vec<T>,
    // inverse element
    inv: Vec<T>,
    // inverse element of factorial
    finv: Vec<T>,
    modulo: T,
}

impl<T> FactorialMod<T>
where
    T: Copy
        + Clone
        + Default
        + num::PrimInt
        + num::Zero
        + num::One
        + num::NumCast
        + num::Signed
        + std::fmt::Debug
        + std::ops::Add
        + std::ops::AddAssign
        + std::ops::Sub
        + std::ops::SubAssign
        + std::ops::Mul
        + std::ops::MulAssign
        + std::ops::Neg
        + std::ops::Rem
        + std::ops::RemAssign,
{
    fn new(modulo: T, n: Option<usize>) -> Self {
        let n = n.unwrap_or(0);

        let mut fact: Vec<T>;
        if n > 0 {
            fact = Vec::with_capacity(n + 1);
        } else {
            fact = Vec::new();
        }
        fact.push(T::one());

        let mut inv: Vec<T>;
        let mut finv: Vec<T>;
        if n > 0 {
            inv = Vec::with_capacity(n + 1);
            finv = Vec::with_capacity(n + 1);
        } else {
            inv = Vec::new();
            finv = Vec::new();
        }
        inv.push(T::zero());
        inv.push(T::one());
        finv.push(T::one());
        finv.push(T::one());

        Self {
            fact,
            inv,
            finv,
            modulo,
        }
    }

    fn mod_pos(&self, x: T) -> T {
        let mut x = x % self.modulo;
        if x < T::zero() {
            x += self.modulo;
        }

        x
    }

    // factorial
    fn fact(&mut self, n: usize) -> T {
        while self.fact.len() <= n {
            let size = self.mod_pos(T::from(self.fact.len()).unwrap());
            let mut val = *self.fact.last().unwrap();
            val *= size;
            val %= self.modulo;
            self.fact.push(val);
        }
        self.fact[n]
    }

    // factorial of inverse element
    // 逆元を求める
    fn finv(&mut self, n: usize) -> T {
        for i in self.inv.len()..=n {
            let mut val = -self.inv[self.modulo.to_usize().unwrap() % i]
                * self.mod_pos(self.modulo / T::from(i).unwrap());
            val = self.mod_pos(val);
            self.inv.push(val);
            self.finv
                .push(self.mod_pos(*self.finv.last().unwrap() * val));
        }
        self.finv[n]
    }
    // binominal coefficient
    fn binom(&mut self, n: usize, k: usize) -> T {
        if n < k {
            return T::zero();
        }
        let f = self.fact(n);
        let fi1 = self.finv(k);
        let fi2 = self.finv(n - k);
        let mut val = self.mod_pos(f * fi1);
        val = self.mod_pos(val * fi2);
        val
    }
}

mod tests {
    use super::{FactorialMod, MOD_INT};

    #[test]
    fn test_fact() {
        let mut fact = FactorialMod::new(MOD_INT as i64, Some(10));
        let answers: Vec<usize> = vec![
            1 % MOD_INT,
            1 % MOD_INT,
            2 % MOD_INT,
            6 % MOD_INT,
            24 % MOD_INT,
            120 % MOD_INT,
            720 % MOD_INT,
            5040 % MOD_INT,
            40320 % MOD_INT,
            362880 % MOD_INT,
            3628800 % MOD_INT,
            39916800 % MOD_INT,
            479001600 % MOD_INT,
            6227020800 % MOD_INT,
            87178291200 % MOD_INT,
            1307674368000 % MOD_INT,
            20922789888000 % MOD_INT,
            355687428096000 % MOD_INT,
            6402373705728000 % MOD_INT,
            121645100408832000 % MOD_INT,
            2432902008176640000 % MOD_INT,
        ];
        for (i, answer) in answers.iter().enumerate() {
            assert_eq!(fact.fact(i) as usize, *answer);
        }
    }
}

fn main() {
    input! {
        in_n: usize,
        in_m: usize,
        in_k: usize,
        in_uv: [(usize, usize); in_m]
    }

    let in_uv = in_uv
        .iter()
        .map(|&(u, v)| (u - 1, v - 1))
        .collect::<Vec<_>>();

    let mut graph = vec![vec![]; in_n];
    for &(u, v) in &in_uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut cnt_odd = 0;
    for v in &graph {
        if v.len() % 2 == 1 {
            cnt_odd += 1;
        }
    }

    let mut fact = FactorialMod::new(MOD_INT as i64, Some(in_n));

    let mut ans: usize = 0;
    // 次数が奇数の点のうち k 個塗るとき
    let cnt_even = in_n - cnt_odd;
    let k_min = std::cmp::max(0, in_k as i64 - cnt_even as i64) as usize;
    let k_max = std::cmp::min(in_k, cnt_odd) as usize;
    for k in (k_min..=k_max).step_by(2) {
        let v1 = fact.binom(cnt_odd, k) as usize;
        let v2 = fact.binom(cnt_even, in_k - k) as usize;
        ans += (v1 * v2) % MOD_INT;
        ans %= MOD_INT;
    }

    println!("{}", ans);

    println!("{}", fact.binom(10, 5));
    println!("{}", fact.binom(10, 0));
    println!("{}", fact.binom(10, 1));
    println!("{}", fact.binom(10, 8));
}

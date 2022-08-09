use proconio::input;

static MOD_INT: usize = 1000000007;

#[derive(Debug, Clone)]
struct Tensor<T> {
    data: Vec<Vec<T>>,
}

impl<T> Tensor<T>
where
    T: Default
        + Clone
        + Copy
        // + std::ops::Deref
        + std::ops::AddAssign
        + std::ops::MulAssign
        + std::ops::SubAssign
        + std::ops::Add
        + std::ops::AddAssign
        + std::ops::Sub
        + std::ops::SubAssign
        + std::ops::Mul
        + std::ops::MulAssign
        + std::ops::Div
        + std::ops::DivAssign
        + std::ops::Rem
        + std::ops::RemAssign
        + num::PrimInt
        + num::Zero
        + num::One
        + num::NumCast,
{
    fn new(n: usize, m: usize, val: std::option::Option<T>) -> Self {
        let val = val.unwrap_or(T::default());
        Tensor {
            data: vec![vec![val; m]; n],
        }
    }

    fn identity(n: usize) -> Self {
        let mut id = Tensor::new(n, n, Some(T::zero()));
        for i in 0..n {
            id.data[i][i] = T::one();
        }
        id
    }

    // 数学的表現
    // t: 行列
    // v: 列ベクトル
    // return: t * v の列ベクトル
    fn mul_vec(&self, v: &Vec<T>) -> Vec<T> {
        let n = self.data.len();
        let m = self.data[0].len();
        let mut res = vec![T::zero(); n];
        for i in 0..n {
            for j in 0..m {
                res[i] += self.data[i][j] * v[j];
                res[i] %= T::from(MOD_INT).unwrap();
            }
        }
        res
    }
}

fn mul_tensor2(t: &Tensor<usize>, t2: &Tensor<usize>) -> Tensor<usize> {
    let n = t.data.len();
    let m = t.data[0].len();

    let mut res = Tensor::new(n, m, Some(0 as usize));
    for i in 0..n {
        for j in 0..m {
            for k in 0..m {
                res.data[i][j] += t.data[i][k] * t2.data[k][j];
                res.data[i][j] %= MOD_INT;
            }
        }
    }
    res
}

// 繰り返し二乗法
// exponentiation by squaring
fn pow_tensor2(t: &Tensor<usize>, n: usize) -> Tensor<usize> {
    let mut n = n;
    let mut res = Tensor::<usize>::identity(t.data.len());
    let mut t = t.clone();
    while n > 0 {
        if n & 1 == 1 {
            res = mul_tensor2(&res, &t);
        }
        t = mul_tensor2(&t, &t);
        n >>= 1;
    }
    res
}

fn print_table<T>(table: &Vec<Vec<T>>)
where
    T: std::fmt::Display,
{
    let row_size = table.len();
    if row_size == 0 {
        return;
    }
    let col_size = table[0].len();
    let mut col_size_vec = vec![0; col_size];
    for row in table.iter() {
        for (c_idx, cell) in row.iter().enumerate() {
            let s = format!("{}", cell);
            col_size_vec[c_idx] = std::cmp::max(col_size_vec[c_idx], s.len());
        }
    }
    for row in table.iter().rev() {
        for (c_idx, cell) in row.iter().enumerate() {
            print!("{:>width$} ", cell, width = col_size_vec[c_idx] + 1);
        }
        println!();
    }
}

fn main() {
    input! {
        in_n: usize,
        in_b: usize,
        in_k: usize,
        in_c: [usize; in_k],
    }

    let mut dp_first = vec![0 as usize; in_b];
    dp_first[0] = 1;

    let mut t = Tensor::new(in_b, in_b, Some(0 as usize));
    for i in 0..in_b {
        for &c in &in_c {
            let rem = (10 * i + c) % in_b;
            t.data[i][rem] += 1;
        }
    }

    // dbg!(&t);
    // print_table(&t.data);

    let dp_last = pow_tensor2(&t, in_n).mul_vec(&dp_first);

    println!("{}", dp_last[0]);
}

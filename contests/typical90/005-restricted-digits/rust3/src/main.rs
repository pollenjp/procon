use proconio::input;

static MOD_INT: usize = 1000000007;

fn modpow<T>(a: T, b: T, m: T) -> T
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::BitAnd<Output = T>
        + std::ops::Mul
        + std::ops::MulAssign
        + std::ops::Rem<Output = T>
        + std::ops::RemAssign
        + std::ops::Shr<Output = T>
        + Ord
        + PartialOrd
        + num::Zero
        + num::One,
{
    let mut a = a;
    let mut b = b;
    let mut res = T::one();
    while b > T::zero() {
        if b & T::one() == T::one() {
            res *= a;
            res %= m;
        }
        a *= a;
        a %= m;
        b = b >> T::one();
    }
    res
}

fn main() {
    input! {
        in_n: usize, // <= 10^18
        in_b: usize,
        in_k: usize,
        in_c: [usize; in_k],
    }

    // 64 bit (in_n の max bit 数)
    // 59 < log( 10^18 ) < 60
    let max_bits = 60;

    let mut power10 = vec![0 as usize; max_bits];
    for i in 0..max_bits {
        power10[i] = modpow(10, 1 << i, in_b);
    }

    let mut dp = vec![vec![0 as usize; in_b]; max_bits + 1];

    // idx 0 が1桁の時のdp
    for &c in &in_c {
        dp[0][c % in_b] += 1;
    }

    // dp[0][i], dp[1][i], dp[2][i], dp[4][i], dp[8][i], ..., dp[2^n][i]
    for i in 0..max_bits {
        for j in 0..in_b {
            for k in 0..in_b {
                let nex = (j * power10[i] + k) % in_b;
                dp[i + 1][nex] += dp[i][j] * dp[i][k];
                dp[i + 1][nex] %= MOD_INT;
            }
        }
    }

    let mut ans = vec![vec![0 as usize; in_b]; max_bits + 1];
    ans[0][0] = 1;
    // exponentiation by squaring (繰り返し二乗法)
    for i in 0..max_bits {
        if in_n & (1 << i) == 0 {
            for j in 0..in_b {
                ans[i + 1][j] = ans[i][j];
            }
            continue;
        }

        for j in 0..in_b {
            for k in 0..in_b {
                let nex = (j * power10[i] + k) % in_b;
                ans[i + 1][nex] += ans[i][j] * dp[i][k];
                ans[i + 1][nex] %= MOD_INT;
            }
        }
    }

    println!("{}", ans[max_bits][0]);
}

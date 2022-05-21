#include <algorithm>
#include <iostream>
#include <vector>

constexpr int64_t kP = 100000007;  // prime number

int64_t calc_mod_division_number(int64_t n) {
  int64_t index = kP - 2;
  int64_t res = 1;
  while (index > 0) {
    if (index & 1) {
      res *= n;
      res %= kP;
    }
    n *= n;
    n %= kP;
    index >>= 1;
  }
  return res;
}

std::vector<int64_t> calc_factorial(int64_t n) {
  std::vector<int64_t> factorial(n + 1);
  factorial[0] = 1;
  for (int64_t i = 1; i <= n; ++i) {
    factorial[i] = factorial[i - 1] * i;
    factorial[i] %= kP;
  }
  return factorial;
}

int main() {
  int32_t in_r{};
  int32_t in_c{};
  int32_t in_a1{};
  int32_t in_a2{};
  int32_t in_b1{};
  int32_t in_b2{};
  std::cin >> in_r >> in_c >> in_a1 >> in_a2 >> in_b1 >> in_b2;

  int32_t row_move_path1 = std::abs(in_b1 - in_a1);
  int32_t row_move_path2 = in_r - std::abs(in_b1 - in_a1);
  int64_t row_move_cnt = std::min(row_move_path1, row_move_path2);
  int32_t col_move_path1 = std::abs(in_b2 - in_a2);
  int32_t col_move_path2 = in_c - std::abs(in_b2 - in_a2);
  int64_t col_move_cnt = std::min(col_move_path1, col_move_path2);

  int64_t n = row_move_cnt + col_move_cnt;

  // n_C_r
  std::vector<int64_t> factorial_list = calc_factorial(n);
  int64_t ans{1};
  ans *= factorial_list[n];
  ans %= kP;
  ans *= calc_mod_division_number(factorial_list[row_move_cnt]);
  ans %= kP;
  ans *= calc_mod_division_number(factorial_list[col_move_cnt]);
  ans %= kP;
  if (row_move_path1 == row_move_path2) {
    ans *= 2;
    ans %= kP;
  }
  if (col_move_path1 == col_move_path2) {
    ans *= 2;
    ans %= kP;
  }
  std::cout << ans << std::endl;
}

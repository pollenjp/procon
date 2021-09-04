#include <cmath>
#include <exception>
#include <iostream>
#include <numeric>
#include <stdexcept>
#include <vector>

#define DIVISION_NUMBER static_cast<long long>(1e+9 + 7)

namespace arc122 {

// template <class T>
// T mod(const T a, const T b) {
//   return a - std::floor(a / b) * b;
// }

long long mod(const long long a, const long long b) { return a - std::floor(a / b) * b; }

template <class T>
T add_mod(const T a, const T b) {
  return mod(mod(a, DIVISION_NUMBER) + mod(b, DIVISION_NUMBER), DIVISION_NUMBER);
}

template <class T>
void dp(T &out_cnt, T &out_sum, const T val_0, const T val_1, const T pre1_num, const T pre1_sum, const T pre2_num,
        const T pre2_sum) {
  T val1 = add_mod(val_0 * pre1_num, pre1_sum);
  T val2 = add_mod(add_mod(val_0, -val_1) * pre2_num, pre2_sum);
  out_cnt = add_mod(pre1_num, pre2_num);
  out_sum = add_mod(val1, val2);
}

template <class T>
T solve(T N, const std::vector<T> &a_vec) {
  std::vector<T> dp_sum_vec;
  std::vector<T> dp_cnt_vec;
  std::vector<T> sub_vec;
#pragma unroll 5
  for (T i = 0; i < N; i++) {
    sub_vec.push_back(*(a_vec.end() - i - 1));
    T sum_val(0);
    T conb_cnt(0);
    if (i >= 2) {
      dp(conb_cnt, sum_val, sub_vec[i], sub_vec[i - 1], dp_cnt_vec[i - 1], dp_sum_vec[i - 1], dp_cnt_vec[i - 2],
         dp_sum_vec[i - 2]);
    } else if (i == 1) {
      sum_val = (2 * (*(a_vec.end() - i - 1))) % DIVISION_NUMBER;
      conb_cnt = 2;
    } else if (i == 0) {
      sum_val = (*(a_vec.end() - i - 1)) % DIVISION_NUMBER;
      conb_cnt = 1;
    } else {
      throw std::runtime_error("error");
    }
    dp_sum_vec.push_back(sum_val);
    dp_cnt_vec.push_back(conb_cnt);
  }

  return *(dp_sum_vec.end() - 1);
}
}  // namespace arc122

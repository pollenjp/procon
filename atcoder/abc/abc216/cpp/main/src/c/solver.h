#include <string>

namespace {

template <class T>
std::string Num2BinStr(const T a) {
  T num = a;
  std::string b_str("");
  while (num != 0) {
    b_str = std::to_string(num % 2) + b_str;
    num /= 2;
  }
  return b_str;
}

template <class T>
std::string solve(const T n) {
  std::string target;  // n >= 1 なので最初は必ずAを選ぶ
  std::string n_bin_str = Num2BinStr(n);

  for (std::size_t i = 0; i < n_bin_str.size(); i++) {
    if (i >= 1) {
      target.push_back('B');
    }
    if (n_bin_str[i] == '1') {
      target.push_back('A');
    }
  }

  return target;
}
}  // namespace

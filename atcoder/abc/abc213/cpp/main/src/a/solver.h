#include <bitset>
#include <string>

namespace {

std::string Num2BinStr(const int a) {
  int num = a;
  std::string b_str("");
  while (num != 0) {
    b_str = std::to_string(num % 2) + b_str;
    num /= 2;
  }
  return b_str;
}

int solve(const int a, const int b) {
  std::bitset<8> a_bin(Num2BinStr(a)), b_bin(Num2BinStr(b));
  return (int)(a_bin ^ b_bin).to_ulong();
}
}  // namespace

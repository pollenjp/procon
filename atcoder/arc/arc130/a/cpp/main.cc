#include <iostream>
#include <set>
#include <string>
#include <vector>

int64_t calcNumOfCombination(int64_t n, int64_t r) {
  int64_t num = 1;
  for (int i = 1; i <= r; i++) {
    num = num * (n - i + 1) / i;
  }
  return num;
}

int main() {
  int64_t num_n(0);
  std::cin >> num_n;

  std::string s_str;
  std::cin >> s_str;

  std::vector<int64_t> cnt_list(num_n, 0);
  std::size_t cnt_list_idx(0);
  char pre_char = s_str[0];
  int64_t current_cnt(1);
  for (int64_t i = 1; i < num_n; i++) {
    if (s_str[i] == pre_char) {
      current_cnt++;
    } else {
      cnt_list[cnt_list_idx] = current_cnt;
      cnt_list_idx++;
      current_cnt = 1;
      pre_char = s_str[i];
    }
  }

  cnt_list[cnt_list_idx] = current_cnt;
  cnt_list_idx++;

  int64_t ans_cnt(0);
  for (std::size_t i = 0; i < cnt_list_idx; i++) {
    if (cnt_list[i] >= 2) {
      ans_cnt += calcNumOfCombination(cnt_list[i], 2);
    }
  }
  std::cout << ans_cnt << std::endl;

  return 0;
}

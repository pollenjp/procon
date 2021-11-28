#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

int main() {
  int32_t num_h(0);
  int32_t num_w(0);
  int32_t num_c(0);
  int32_t num_q(0);
  std::cin >> num_h >> num_w >> num_c >> num_q;

  std::vector<int32_t> t_list(num_q);
  std::vector<int32_t> n_list(num_q);
  std::vector<int32_t> c_list(num_q);
  for (int32_t i(0); i < num_q; ++i) {
    std::cin >> t_list[i] >> n_list[i] >> c_list[i];
  }

  std::vector<int64_t> color_cnt_list(num_c, 0);
  std::map<int32_t, int32_t> row_cnt_map;
  std::map<int32_t, int32_t> column_cnt_map;
  // for (int32_t i(1); i <= num_c; ++i) {
  //   color_cnt_map[i] = 0;
  // }

  int32_t h_now(num_h);
  int32_t w_now(num_w);
  int32_t t, n, c;
  for (int32_t i(num_q - 1); i >= 0; i--) {
    t = t_list[i];
    n = n_list[i];
    c = c_list[i];
    if (t == 1) {
      if (row_cnt_map[n] == 0) {
        row_cnt_map[n]++;
        color_cnt_list[c - 1] += w_now;
        h_now--;
      }
    } else {  // t == 2
      if (column_cnt_map[n] == 0) {
        column_cnt_map[n]++;
        color_cnt_list[c - 1] += h_now;
        w_now--;
      }
    }
  }

  for (int32_t i = 0; i < num_c; i++) {
    std::cout << color_cnt_list[i] << " ";
  }

  return 0;
}

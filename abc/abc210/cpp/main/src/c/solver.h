#include <vector>

namespace {

void insert_unknown_color(const int color,
                          std::vector<std::pair<int, int>> &stock) {
  unsigned int stock_idx;
  bool is_already_known_color = false;

  for (stock_idx = 0; stock_idx < stock.size(); stock_idx++) {
    if (stock[stock_idx].first == color) {
      is_already_known_color = true;
    }
  }
  if (!is_already_known_color) {
    stock.push_back({color, 1});
  }
}

unsigned int solve(const int N, const int K, const std::vector<int> &candies) {
  unsigned int max_num_of_colors = 0;
  std::vector<std::pair<int, int>> stock({});  // color, counter

  unsigned int idx;

  // init
  for (idx = 0; idx < candies.size(); idx++) {
    insert_unknown_color(candies[idx], stock);
  }
  max_num_of_colors = stock.size();

  // main
  for (idx = 1; idx < static_cast<unsigned int>(N - K + 1); idx++) {
    // remove from stock
    int tmp_color = candies[idx - 1];
    unsigned int stock_idx;
    for (stock_idx = 0; stock_idx < stock.size(); stock_idx++) {
      if (stock[stock_idx].first == tmp_color) {
        if (stock[stock_idx].second == 1) {
          // count is 1
          stock.erase(stock.begin() + stock_idx);
        } else {
          stock[stock_idx].second--;
        }
        break;
      }
    }

    // insert next color to stock
    insert_unknown_color(tmp_color, stock);

    if (stock.size() >= max_num_of_colors) {
      max_num_of_colors = stock.size();
    }
  }

  return max_num_of_colors;
}
}  // namespace

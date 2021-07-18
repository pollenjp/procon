#include <map>
#include <vector>

namespace {

void insert_unknown_color(const int color, std::map<int, int> &stock) {
  stock[color]++;
}

unsigned int solve(const int N, const int K, const std::vector<int> &candies) {
  unsigned int max_num_of_colors = 0;
  std::map<int, int> stock({});  // key:color, val:counter

  unsigned int idx;

  // init
  for (idx = 0; idx < static_cast<unsigned int>(K); idx++) {
    insert_unknown_color(candies[idx], stock);
  }
  max_num_of_colors = stock.size();

  // main
  for (idx = 1; idx < static_cast<unsigned int>(N - K + 1); idx++) {
    // remove from stock
    int previous_color = candies[idx - 1];
    stock[previous_color]--;
    if (stock[previous_color] == 0) {
      stock.erase(previous_color);
    }

    // insert next color to stock
    int next_color = candies[idx + K - 1];
    insert_unknown_color(next_color, stock);

    if (stock.size() > max_num_of_colors) {
      max_num_of_colors = stock.size();
    }
  }

  return max_num_of_colors;
}
}  // namespace

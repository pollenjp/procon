#include <deque>
#include <functional>
#include <iostream>
#include <random>
#include <set>
#include <unordered_set>
#include <vector>

struct HashPair {
  static size_t m_hash_pair_random_;

  template <class T1, class T2>
  std::size_t operator()(const std::pair<T1, T2> &p) const {
    auto hash1 = std::hash<T1>()(p.first);
    auto hash2 = std::hash<T2>()(p.second);

    std::size_t seed = 0;
    seed ^= hash1 + 0x9e3779b9 + (seed << 6) + (seed >> 2);
    seed ^= hash2 + 0x9e3779b9 + (seed << 6) + (seed >> 2);
    seed ^= HashPair::m_hash_pair_random_ + 0x9e3779b9 + (seed << 6) + (seed >> 2);
    return seed;
  }
};

std::size_t HashPair::m_hash_pair_random_ = static_cast<std::size_t>(std::random_device()());

int main() {
  int32_t num_h(0);
  int32_t num_w(0);
  std::cin >> num_h >> num_w;

  std::vector<std::vector<char>> c_tile(num_h + 1, std::vector<char>(num_w + 1));

  for (int32_t row_idx(0); row_idx < num_h; row_idx++) {
    for (int32_t column_idx(0); column_idx < num_w; column_idx++) {
      std::cin >> c_tile[row_idx][column_idx];
    }
    c_tile[row_idx][num_w] = '#';
  }
  for (auto &c : c_tile[num_h]) {
    c = '#';
  }

  std::vector<std::vector<int32_t>> max_cost_tile(num_h, std::vector<int>(num_w));
  max_cost_tile[0][0] = 1;

  std::deque<std::pair<int32_t, int32_t>> vertex_deque;
  std::unordered_set<std::pair<int32_t, int32_t>, HashPair> visited_set;

  int32_t max_cost(1);

  int32_t row_idx = 0;
  int32_t col_idx = 0;
  vertex_deque.emplace_back(std::make_pair(row_idx, col_idx));
  while (!vertex_deque.empty()) {
    row_idx = vertex_deque.front().first;
    col_idx = vertex_deque.front().second;
    vertex_deque.pop_front();

    if (c_tile[row_idx + 1][col_idx] != '#') {
      max_cost_tile[row_idx + 1][col_idx] = max_cost_tile[row_idx][col_idx] + 1;
      max_cost = std::max(max_cost, max_cost_tile[row_idx + 1][col_idx]);
      if (visited_set.find(std::make_pair(row_idx + 1, col_idx)) == visited_set.end()) {
        vertex_deque.emplace_back(std::make_pair(row_idx + 1, col_idx));
        visited_set.emplace(std::make_pair(row_idx + 1, col_idx));
      }
    }

    if (c_tile[row_idx][col_idx + 1] != '#') {
      max_cost_tile[row_idx][col_idx + 1] = max_cost_tile[row_idx][col_idx] + 1;
      max_cost = std::max(max_cost, max_cost_tile[row_idx][col_idx + 1]);
      if (visited_set.find(std::make_pair(row_idx, col_idx + 1)) == visited_set.end()) {
        vertex_deque.emplace_back(std::make_pair(row_idx, col_idx + 1));
        visited_set.emplace(std::make_pair(row_idx, col_idx + 1));
      }
    }
  }

  std::cout << max_cost << std::endl;

  return 0;
}

#include <algorithm>
#include <iostream>
#include <map>
#include <vector>

namespace {

/**
 * 都市の名前は 0 ~ (N-1) とする
 */
std::vector<int> solve(int n, std::vector<std::pair<int, int>> x_vec) {
  std::vector<std::vector<int>> node_vec(n, std::vector<int>());

  for (auto itr = x_vec.begin(); itr != x_vec.end(); ++itr) {
    node_vec[itr->first - 1].push_back(itr->second - 1);
    node_vec[itr->second - 1].push_back(itr->first - 1);
  }

  // sort node_vec's vector
  for (std::size_t i = 0; i < node_vec.size(); ++i) {
    std::sort(node_vec[i].begin(), node_vec[i].end());
  }

  int current_node(0);
  std::vector<int> track({current_node});
  int next_node(-1);
  while (true) {
    std::vector<int> candidate_nodes = node_vec[current_node];
    for (std::size_t i = 0; i < candidate_nodes.size(); ++i) {
      std::cout << current_node << " " << candidate_nodes[i] << std::endl;
      if (std::find(track.begin(), track.end(), candidate_nodes[i]) ==
          track.end()) {
        // not present
        next_node = candidate_nodes[i];
        break;
      }
    }
    if (next_node == -1) {
      if (current_node == 0) {
        break;
      }
      if (track.size() < 2) {
        break;
      }
      current_node = track[track.size() - 2];
    } else {
      current_node = next_node;
      track.push_back(next_node);
    }
    next_node = -1;
  }

  return track;
}
}  // namespace

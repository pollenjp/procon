#include <bits/stdc++.h>

void debug_print(const std::vector<std::unordered_set<int32_t>> &connect_set_list) {
  for (const auto &connect_set : connect_set_list) {
    for (const auto &connect_set_element : connect_set) {
      std::cout << connect_set_element << " ";
    }
    std::cout << std::endl;
  }
}

class Node {
 public:
  int32_t parent;
  int32_t parent_min;
  std::unordered_set<int32_t> children;
  std::pair<int32_t, int32_t> lr;

  Node() : parent(-1), parent_min(-1) {}
};

void debug_print(const Node &node) {
  std::cout << "parent_min: " << node.parent_min << std::endl;
  std::cout << "children: ";
  for (const auto &child_idx : node.children) {
    std::cout << child_idx << " ";
  }
  std::cout << std::endl;
  std::cout << "lr: " << node.lr.first << " " << node.lr.second << std::endl;
}

void create_node(int32_t current_node_idx, std::vector<std::unordered_set<int32_t>> &connect_set_list,
                 std::vector<Node> &node_list) {
  if (connect_set_list[current_node_idx].empty()) {
    return;
  }
  auto connect_nodes = connect_set_list[current_node_idx];
  for (const auto &child_idx : connect_nodes) {
    connect_set_list[current_node_idx].erase(child_idx);
    connect_set_list[child_idx].erase(current_node_idx);

    node_list[current_node_idx].children.insert(child_idx);
    if (node_list[child_idx].parent == -1) {
      node_list[child_idx].parent = current_node_idx;
      node_list[current_node_idx].children.insert(child_idx);
      create_node(child_idx, connect_set_list, node_list);
    }
  }
}

void set_lr(const int32_t current_node_idx, std::vector<std::unordered_set<int32_t>> &connect_set_list,
            std::vector<Node> &node_list) {
  auto &current_node = node_list[current_node_idx];

  if (current_node.children.empty()) {
    current_node.lr.first = current_node.parent_min;
    current_node.lr.second = current_node.parent_min;
    return;
  }

  auto min_val = current_node.parent_min;
  for (const auto &child_idx : current_node.children) {
    auto &child_node = node_list[child_idx];

    child_node.parent = current_node_idx;
    child_node.parent_min = min_val;

    set_lr(child_idx, connect_set_list, node_list);

    min_val = child_node.lr.second + 1;
  }
  current_node.lr.first = current_node.parent_min;
  current_node.lr.second = min_val - 1;
}

int main() {
  int32_t num_n(0);
  std::cin >> num_n;

  std::vector<std::pair<int32_t, int32_t>> uv_list(num_n - 1);
  std::vector<std::unordered_set<int32_t>> connect_set_list(num_n);

  for (auto &uv : uv_list) {
    std::cin >> uv.first >> uv.second;
    connect_set_list[uv.first - 1].insert(uv.second - 1);
    connect_set_list[uv.second - 1].insert(uv.first - 1);
  }

#ifdef DEBUG
  debug_print(connect_set_list);
#endif

  std::vector<Node> node_list(num_n);
  node_list[0].parent_min = 1;

#ifdef DEBUG
  for (auto &node : node_list) {
    debug_print(node);
  }
#endif

  // create nodes
  create_node(0, connect_set_list, node_list);

#ifdef DEBUG
  for (auto &node : node_list) {
    debug_print(node);
  }
#endif

  set_lr(0, connect_set_list, node_list);

#ifdef DEBUG
  for (auto &node : node_list) {
    debug_print(node);
  }
#endif

  for (const auto &node : node_list) {
    std::cout << node.lr.first << " " << node.lr.second << std::endl;
  }

  return 0;
}

#include <bits/stdc++.h>

constexpr int32_t kMaxK = 20;

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
  int32_t value;
  int32_t parent;
  int32_t parent_min;
  std::unordered_set<int32_t> children;
  std::vector<int32_t> inv_sorted_values;

  explicit Node(int32_t value) : value(value), parent(-1), parent_min(-1) {}
};

void debug_print(const Node &node) {
  std::cout << "value: " << node.value << std::endl;
  std::cout << "parent_min: " << node.parent_min << std::endl;
  std::cout << "children: ";
  for (const auto &child_idx : node.children) {
    std::cout << child_idx << " ";
  }
  std::cout << std::endl;
  std::cout << "inv_sorted_values: ";
  for (const auto &val : node.inv_sorted_values) {
    std::cout << val << " ";
  }
  std::cout << std::endl;
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

void set_sort_list(const int32_t current_node_idx, std::vector<std::unordered_set<int32_t>> &connect_set_list,
                   std::vector<Node> &node_list) {
  auto &current_node = node_list[current_node_idx];

  if (current_node.children.empty()) {
    current_node.inv_sorted_values.push_back(current_node.value);
    return;
  }

  auto value_list = std::vector<int32_t>();
  value_list.push_back(current_node.value);
  for (const auto &child_idx : current_node.children) {
    auto &child_node = node_list[child_idx];
    set_sort_list(child_idx, connect_set_list, node_list);
    value_list.insert(value_list.end(), child_node.inv_sorted_values.begin(), child_node.inv_sorted_values.end());
  }
  std::sort(value_list.begin(), value_list.end(), [](int32_t a, int32_t b) { return a > b; });
  // sort
  current_node.inv_sorted_values = std::vector<int32_t>(
      value_list.begin(), value_list.begin() + std::min(kMaxK, static_cast<int32_t>(value_list.size())));
}

int main() {
  int32_t num_n(0);
  int32_t num_q(0);
  std::cin >> num_n >> num_q;

  std::vector<int32_t> x_list(num_n);
  for (auto &x : x_list) {
    std::cin >> x;
  }

  std::vector<std::pair<int32_t, int32_t>> ab_list(num_n - 1);
  std::vector<std::unordered_set<int32_t>> connect_set_list(num_n);

  for (auto &ab : ab_list) {
    std::cin >> ab.first >> ab.second;
    connect_set_list[ab.first - 1].insert(ab.second - 1);
    connect_set_list[ab.second - 1].insert(ab.first - 1);
  }

#ifdef DEBUG
  std::cout << "================================================" << std::endl;
  debug_print(connect_set_list);
#endif

  std::vector<Node> node_list(num_n, Node(0));
  for (int32_t i(0); i < num_n; i++) {
    auto &node = node_list[i];
    node.value = x_list[i];
  }
  node_list[0].parent_min = 1;

#ifdef DEBUG
  std::cout << "================================================" << std::endl;
  for (auto &node : node_list) {
    debug_print(node);
  }
#endif

  // create nodes
  create_node(0, connect_set_list, node_list);

#ifdef DEBUG
  std::cout << "================================================" << std::endl;
  for (auto &node : node_list) {
    debug_print(node);
  }
#endif

  set_sort_list(0, connect_set_list, node_list);

#ifdef DEBUG
  std::cout << "================================================" << std::endl;
  for (auto &node : node_list) {
    debug_print(node);
  }
#endif

  for (int32_t i(0); i < num_q; i++) {
    int32_t k(0);
    int32_t v(0);
    std::cin >> k >> v;
    auto &node_k = node_list[k - 1];
    std::cout << node_k.inv_sorted_values[v - 1] << std::endl;
  }

  return 0;
}

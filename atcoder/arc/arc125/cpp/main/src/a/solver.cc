#include "solver.h"

int arc215::solve(std::vector<int> &s_vec, std::vector<int> &t_vec) {
  if (s_vec.empty()) {
    throw std::invalid_argument("s_vec is empty");
  }

  if (t_vec.empty()) {
    throw std::invalid_argument("t_vec is empty");
  }

  std::vector<int> s_vec_element_set;
  s_vec_element_set.push_back(s_vec[0]);
  for (auto &s_i : s_vec) {
    if (s_i != s_vec_element_set[0]) {
      s_vec_element_set.push_back(s_i);
      break;
    }
  }

  std::vector<int> t_vec_element_set;
  t_vec_element_set.push_back(t_vec[0]);
  for (auto &t_i : t_vec) {
    if (t_i != t_vec_element_set[0]) {
      t_vec_element_set.push_back(t_i);
      break;
    }
  }

  std::sort(s_vec_element_set.begin(), s_vec_element_set.end());
  std::sort(t_vec_element_set.begin(), t_vec_element_set.end());

  if (s_vec_element_set.size() == 1) {
    if (t_vec_element_set[0] == s_vec_element_set[0] && t_vec_element_set.size() == 1) {
      return static_cast<int>(t_vec.size());
    }
    // impossible
    return -1;
  }
  if (s_vec_element_set.empty()) {
    throw std::runtime_error("should have at least one element");
  }

  // a_1 が s_vec から一番少ないシフト数で異なる値になる回数は？
  int num_shift = static_cast<int>(s_vec.size());
  for (std::size_t i = 0; i < s_vec.size(); ++i) {
    if (s_vec[i] != s_vec[0]) {
      int tmp = std::min(i, static_cast<int>(s_vec.size()) - i);
      num_shift = std::min(num_shift, tmp);
    }
  }

  int cnt(0);
  int a_0(s_vec[0]);
  bool is_first(true);
  for (auto &t_i : t_vec) {
    if (t_i == a_0) {
      cnt++;
    } else {
      if (is_first) {
        cnt += num_shift;
        is_first = false;
      } else {
        cnt++;
      }
      a_0 = t_i;
      cnt++;
    }
  }

  return cnt;
}

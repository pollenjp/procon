#include <iostream>
#include <vector>
#include <set>

int main() {
  int32_t num_n(0);
  int32_t num_k(0);
  std::cin >> num_n >> num_k;

  std::vector<int32_t> p_list(num_n);

  for (int32_t &p_i : p_list) {
    std::cin >> p_i;
  }

  std::set<int32_t> p_set;

  std::set<int32_t>::iterator p_set_it = p_set.begin();
  int32_t top_k_min(num_n+1);
  for (int32_t i(0); i < num_n; i++) {
    p_set.insert(p_list[i]);
    if (i <= (num_k-1)){
      top_k_min = std::min(top_k_min, p_list[i]);
    }
    if (i == (num_k-1)) {
      p_set_it = p_set.find(top_k_min);
      std::cout << (*p_set_it) << std::endl;
    }
    else if (i > (num_k-1)) {
      if (p_list[i] < *p_set_it)
      {
        std::cout << (*p_set_it) << std::endl;
      }
      else{ // == は無い
        p_set_it  = p_set.upper_bound(*p_set_it);
        std::cout << (*p_set_it) << std::endl;
      }
    }
  }

  return 0;
}

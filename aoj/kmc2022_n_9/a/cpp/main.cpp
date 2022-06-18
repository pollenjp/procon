#include <iostream>
#include <vector>

using std::cin;
using std::cout;
using std::endl;

int main() {
  while (true) {
    int32_t in_n(0);
    int32_t in_m(0);
    cin >> in_n >> in_m;
    if (in_n == 0 && in_m == 0) {
      break;
    }
    auto a_vec = std::vector<int32_t>(in_n);
    for (auto &a : a_vec) {
      cin >> a;
    }

    int32_t best_sum(0);

    for (int32_t i = 0; i < in_n - 1; ++i) {
      for (int32_t j = i + 1; j < in_n; ++j) {
        int32_t sum = a_vec[i] + a_vec[j];
        if (best_sum < sum && sum <= in_m) {
          best_sum = sum;
        }
      }
    }

    if (best_sum == 0) {
      cout << "NONE" << endl;
    } else {
      cout << best_sum << endl;
    }
  }
  return EXIT_SUCCESS;
}

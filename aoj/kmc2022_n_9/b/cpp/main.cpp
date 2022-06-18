#include <iostream>
#include <numeric>
#include <vector>

using std::cin;
using std::cout;
using std::endl;

int main() {
  while (true) {
    int32_t in_n(0);
    cin >> in_n;
    if (in_n == 0) {
      break;
    }
    auto a_vec = std::vector<int32_t>(in_n);
    for (auto &a : a_vec) {
      cin >> a;
    }

    int32_t sum = std::accumulate(a_vec.begin(), a_vec.end(), 0);
    int32_t max_x = sum / in_n;

    int32_t cnt(0);

    for (auto &a : a_vec) {
      if (a <= max_x) {
        cnt++;
      }
    }

    cout << cnt << endl;
  }
  return EXIT_SUCCESS;
}

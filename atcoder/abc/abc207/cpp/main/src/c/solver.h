
#include <algorithm>
#include <cmath>
#include <iostream>
#include <vector>

namespace {

int solve(std::vector<std::vector<int>> vec) {
  unsigned int i, j;
  for (auto itr = vec.begin(); itr != vec.end(); itr++) {
    (*itr)[1] *= 2;
    (*itr)[2] *= 2;

    switch ((*itr)[0]) {
      case 1:
        break;
      case 2:
        (*itr)[2]--;
        break;
      case 3:
        (*itr)[1]++;
        break;
      case 4:
        (*itr)[1]++;
        (*itr)[2]--;
        break;
    }
  }

  std::sort(vec.begin(), vec.end(),
            [](const std::vector<int> &a, std::vector<int> &b) {
              return (a[1] < b[1]);
            });

  int cnt = 0;
  for (i = 0; i < vec.size() - 1; i++) {
    for (j = i + 1; j < vec.size(); j++) {
      // std::cout << "(" << vec[i][1] << ", " << vec[i][2] << "), (" <<
      // vec[j][1]
      //           << ", " << vec[j][2] << ")" << std::endl;
      if (vec[j][1] <= vec[i][2]) {
        cnt++;
      } else {
        break;
      }
    }
  }
  return cnt;
}
}  // namespace

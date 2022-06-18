#include <iostream>
#include <numeric>
#include <string>
#include <vector>

using std::cin;
using std::cout;
using std::endl;
using std::string;

std::vector<int32_t> find_char_indices(const string &s, const char &c) {
  auto indices = std::vector<int32_t>{};
  for (int i = 0; i < s.size(); ++i) {
    if (s[i] == c) {
      indices.push_back(i);
    }
  }
  return indices;
}

int main() {
  while (true) {
    string in_s1;
    string in_s2;
    cin >> in_s1;
    if (in_s1 == ".") {
      break;
    }
    cin >> in_s2;

    if (in_s1 == in_s2) {
      cout << "IDENTICAL" << endl;
      continue;
    }
    auto indices1 = find_char_indices(in_s1, '"');
    auto indices2 = find_char_indices(in_s2, '"');
    if (indices1.size() != indices2.size()) {
      cout << "DIFFERENT" << endl;
      continue;
    }

    //

    bool diff_flag = false;
    for (int32_t i(0); i <= indices1.size(); i += 2) {
      int32_t idx1_1(0);
      int32_t idx1_2(0);
      int32_t idx2_1(0);
      int32_t idx2_2(0);
      if (i == 0) {
        idx1_1 = 0;
        idx2_1 = 0;
      } else {
        idx1_1 = indices1[i - 1];
        idx2_1 = indices2[i - 1];
      }

      if (i == indices1.size()) {
        idx1_2 = static_cast<int32_t>(in_s1.size()) - 1;
        idx2_2 = static_cast<int32_t>(in_s2.size()) - 1;
      } else {
        idx1_2 = indices1[i];
        idx2_2 = indices2[i];
      }

      auto x1 = in_s1.substr(idx1_1, idx1_2 - idx1_1);
      auto x2 = in_s2.substr(idx2_1, idx2_2 - idx2_1);
      // cout << "============================" << endl;
      // cout << idx1_1 << "-" << idx1_2 << " : " << idx2_1 << "-" << idx2_2 << endl;
      // cout << x1 << " : " << x2 << endl;
      // cout << "============================" << endl;
      if (x1 != x2) {
        diff_flag = true;
        break;
      }
    }
    if (diff_flag) {
      cout << "DIFFERENT" << endl;
      continue;
    }

    //

    int32_t cnt(0);
    for (int32_t i(0); i < indices1.size(); i += 2) {
      int32_t idx1_1 = indices1.at(i);
      int32_t idx1_2 = indices1.at(i + 1);
      int32_t idx2_1 = indices2.at(i);
      int32_t idx2_2 = indices2.at(i + 1);
      auto x1 = in_s1.substr(idx1_1, idx1_2 - idx1_1);
      auto x2 = in_s2.substr(idx2_1, idx2_2 - idx2_1);
      // cout << "============================" << endl;
      // cout << idx1_1 << "-" << idx1_2 << " : " << idx2_1 << "-" << idx2_2 << endl;
      // cout << x1 << " : " << x2 << endl;
      // cout << "============================" << endl;
      if (x1 != x2) {
        cnt++;
      }
    }

    // cout << "cnt: " << cnt << endl;

    if (cnt <= 1) {
      cout << "CLOSE" << endl;
    } else {
      cout << "DIFFERENT" << endl;
    }
  }
  return EXIT_SUCCESS;
}

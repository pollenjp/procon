#include <iostream>
#include <numeric>
#include <string>
#include <vector>

using std::cin;
using std::cout;
using std::endl;
using std::string;

void cout_vec(const std::vector<int32_t> &v) {
  for (const auto i : v) {
    cout << i << " ";
  }
  cout << endl;
}

void cout_vec2(const std::vector<std::vector<int32_t>> &v2) {
  for (const auto &v : v2) {
    cout_vec(v);
  }
}

int main() {
  while (true) {
    int32_t in_n(0);
    int32_t in_m(0);
    int32_t in_t(0);
    int32_t in_p(0);
    cin >> in_n >> in_m >> in_t >> in_p;
    if (in_n == 0 && in_m == 0 && in_t == 0 && in_p == 0) {
      break;
    }

    auto dc_vec = std::vector<std::pair<int32_t, int32_t>>(in_t);
    for (auto &dc : dc_vec) {
      cin >> dc.first >> dc.second;
    }
    auto xy_vec = std::vector<std::pair<int32_t, int32_t>>(in_p);
    for (auto &xy : xy_vec) {
      cin >> xy.first >> xy.second;
    }

    auto vec1 = std::vector<std::vector<int32_t>>(in_n, std::vector<int32_t>(in_m, 1));
    auto vec2 = std::vector<std::vector<int32_t>>(in_n, std::vector<int32_t>(in_m, 0));

    int32_t width(in_n);
    int32_t height(in_m);
    for (auto &dc : dc_vec) {
      auto d = dc.first;
      auto c = dc.second;
      // cout << "d: " << d << ", c: " << c << endl;
      if (d == 1) {
        auto left = c;
        auto right = width - c;
        // cout << "left: " << left << ", right: " << right << endl;
        auto new_width = std::max(left, right);
        // cout << "new_width: " << new_width << endl;
        for (int32_t i(0); i < new_width; i++) {
          for (int32_t y(0); y < height; y++) {
            auto l_x = c - (1 + i);
            auto r_x = c + i;
            // cout << "l_x: " << l_x << ", r_x: " << r_x << endl;

            int32_t l(0);
            if (l_x < 0) {
              l = 0;
            } else {
              l = vec1.at(c - (1 + i)).at(y);
            }

            int32_t r(0);
            if (r_x > width - 1) {
              r = 0;
            } else {
              r = vec1.at(r_x).at(y);
            }

            vec2[i][y] = l + r;
          }
        }
        width = new_width;
      } else {  // d == 2
        auto lower = c;
        auto upper = height - c;
        auto new_height = std::max(upper, lower);
        for (int32_t i(0); i < new_height; i++) {
          for (int32_t x(0); x < width; x++) {
            // lower_y
            auto l_y = c - (1 + i);
            auto u_y = c + i;

            int32_t l(0);
            if (l_y < 0) {
              l = 0;
            } else {
              l = vec1.at(x).at(l_y);
            }

            int32_t u(0);  // upper
            if (u_y > height - 1) {
              u = 0;
            } else {
              u = vec1.at(x).at(u_y);
            }

            vec2[x][i] = l + u;
          }
        }
        height = new_height;
      }

      vec1 = vec2;
      vec2 = std::vector<std::vector<int32_t>>(width, std::vector<int32_t>(height, 0));

      // cout_vec2(vec1);
      // cout << "-------------------------" << endl;
    }
    for (auto &xy : xy_vec) {
      int32_t cnt = vec1.at(xy.first).at(xy.second);
      cout << cnt << endl;
    }
    // cout << "=========================" << endl;
  }
  return EXIT_SUCCESS;
}

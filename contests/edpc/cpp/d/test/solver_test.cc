#include "pollenlib/solver.h"

#include <string>
#include <vector>

#include "gtest/gtest.h"

namespace atcoder {

namespace edpc {

namespace d {

namespace {

/**
 * @brief split string by delimiter
 *
 * @param str target string
 * @param del delemiter string
 * @return std::vector<std::string> splitted string
 */
std::vector<std::string> split(std::string str, std::string del) {
  std::size_t first = 0;
  std::size_t last = str.find_first_of(del);

  std::vector<std::string> result;

  while (first < str.size()) {
    std::string sub_str(str, first, last - first);

    result.push_back(sub_str);

    first = last + 1;
    last = str.find_first_of(del, first);

    if (last == std::string::npos) {
      last = str.size();
    }
  }

  return result;
}

/**
 * @brief convert lines to vectors
 *
 * @param lines
 * @param w_vec
 * @param v_vec
 */
void read_lines(std::vector<std::string> const &lines, std::vector<int32_t> &w_vec, std::vector<int32_t> &v_vec) {
  std::string delimiter(" ");
  for (std::size_t i = 0; i < lines.size(); i++) {
    auto line = lines[i];
    auto val_vec = split(line, delimiter);
    w_vec[i] = std::stoi(val_vec[0]);
    v_vec[i] = std::stoi(val_vec[1]);
  }
}

class InData {
 public:
  int32_t num_n;
  int32_t num_w;
  std::vector<std::string> lines;
  std::vector<int32_t> w_vec;
  std::vector<int32_t> v_vec;

  InData(std::vector<std::string> lines) {
    auto val_vec = split(lines[0], " ");
    this->num_n = std::stoi(val_vec[0]);
    this->num_w = std::stoi(val_vec[1]);
    this->lines = std::vector<std::string>(lines.begin() + 1, lines.end());

    this->w_vec = std::vector<int32_t>(num_n, 0);
    this->v_vec = std::vector<int32_t>(num_n, 0);
    read_lines(this->lines, this->w_vec, this->v_vec);
  }
};

}  // namespace

TEST(SolverCTest, Sample1) {
  int32_t num_n(3);
  int32_t num_w(8);
  std::vector<std::string> lines({
      "3 30",
      "4 50",
      "5 60",
  });

  std::vector<int32_t> w_vec(num_n, 0);
  std::vector<int32_t> v_vec(num_n, 0);

  read_lines(lines, w_vec, v_vec);

  EXPECT_EQ(solve(num_n, num_w, w_vec, v_vec), 90);
}

TEST(SolverCTest, Sample1_1) {
  InData in_data(std::vector<std::string>({
      "3 8",
      "3 30",
      "4 50",
      "5 60",
  }));

  EXPECT_EQ(solve(in_data.num_n, in_data.num_w, in_data.w_vec, in_data.v_vec), 90);
}

}  // namespace d

}  // namespace edpc

}  // namespace atcoder

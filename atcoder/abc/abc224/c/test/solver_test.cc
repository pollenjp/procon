#include "pollenlib/solver.h"

#include <vector>

#include "gtest/gtest.h"

namespace atcoder {

namespace abc224 {

namespace c {

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
void read_lines(std::vector<std::string> const &lines, std::vector<std::vector<int64_t>> &xy_list) {
  std::string delimiter(" ");
  for (std::size_t i = 0; i < lines.size(); i++) {
    auto line = lines[i];
    auto val_vec = split(line, delimiter);
    xy_list[i][0] = std::stoi(val_vec[0]);
    xy_list[i][1] = std::stoi(val_vec[1]);
    LOG(INFO) << "xy_list: " << xy_list[i][0] << "," << xy_list[i][1];
  }
}

class InData {
 public:
  int32_t num_n;
  std::vector<std::vector<int64_t>> xy_list;

  InData(std::vector<std::string> const &lines) {
    auto val_vec = split(lines[0], " ");

    this->num_n = std::stoi(val_vec[0]);

    auto sub_lines = std::vector<std::string>(lines.begin() + 1, lines.end());

    this->xy_list = std::vector<std::vector<int64_t>>(num_n, std::vector<int64_t>(2, 0));
    read_lines(sub_lines, this->xy_list);
  }
};

}  // namespace

TEST(SolverCTest, Sample1) {
  InData in_data(std::vector<std::string>({
      "4",
      "0 1",
      "1 3",
      "1 1",
      "-1 -1",
  }));

  LOG(INFO) << "num_n: " << in_data.num_n;
  LOG(INFO) << "in_data.xy_list.size(): " << in_data.xy_list.size();

  EXPECT_EQ(solve(in_data.xy_list), 3);
}

}  // namespace c

}  // namespace abc224

}  // namespace atcoder

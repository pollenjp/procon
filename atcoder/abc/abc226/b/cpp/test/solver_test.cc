#include "pollenlib/solver.h"

#include <string>
#include <vector>

#include "gtest/gtest.h"

namespace atcoder {

namespace abc226 {

namespace b {

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
 * @param l_list_list out parameter
 */
void read_lines(std::vector<std::string> const &lines, std::vector<std::vector<int32_t>> &l_list_list) {
  std::string delimiter(" ");
  for (std::size_t i = 0; i < lines.size(); i++) {
    auto line = lines[i];
    auto val_list = split(line, delimiter);
    int32_t num_l(std::stoi(val_list[0]));
    std::vector<int32_t> l_list(num_l + 1);
    for (std::size_t j = 0; j < num_l + 1; j++) {
      l_list[j] = std::stoi(val_list[j]);
    }
    l_list_list[i] = l_list;
  }
}

class InData {
 public:
  int32_t num_n;
  std::vector<std::vector<int32_t>> l_list_list;

  InData(std::vector<std::string> const &lines) {
    auto val_vec = split(lines[0], " ");

    this->num_n = std::stoi(val_vec[0]);

    auto sub_lines = std::vector<std::string>(lines.begin() + 1, lines.end());

    this->l_list_list = std::vector<std::vector<int32_t>>(num_n);

    read_lines(sub_lines, this->l_list_list);
  }
};

}  // namespace

TEST(SolverCTest, Sample1) {
  InData in_data(std::vector<std::string>({
      "4",
      "2 1 2",
      "2 1 1",
      "2 2 1",
      "2 1 2",
  }));

  EXPECT_EQ(solve(in_data.l_list_list), 3);
}

TEST(SolverCTest, Sample2) {
  InData in_data(std::vector<std::string>({
      "5",
      "1 1",
      "1 1",
      "1 2",
      "2 1 1",
      "3 1 1 1",
  }));

  EXPECT_EQ(solve(in_data.l_list_list), 4);
}

TEST(SolverCTest, Sample3) {
  InData in_data(std::vector<std::string>({
      "1",
      "1 1",
  }));

  EXPECT_EQ(solve(in_data.l_list_list), 1);
}

}  // namespace b

}  // namespace abc226

}  // namespace atcoder

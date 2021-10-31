#include "pollenlib/solver.h"

#include <string>
#include <vector>

#include "gtest/gtest.h"

namespace atcoder {

namespace typical90 {

namespace a {

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

class InData {
 public:
  int32_t num_n;
  int32_t num_l;
  int32_t num_k;
  std::vector<int32_t> a_list;

  InData(std::vector<std::string> lines) {
    {
      auto val_vec = split(lines[0], " ");
      LOG(INFO) << "val_vec.size()=" << val_vec.size();
      this->num_n = std::stoi(val_vec[0]);
      this->num_l = std::stoi(val_vec[1]);
    }
    {
      auto val_vec = split(lines[1], " ");
      LOG(INFO) << "val_vec.size()=" << val_vec.size();
      this->num_k = std::stoi(val_vec[0]);
    }
    {
      auto val_vec = split(lines[2], " ");
      LOG(INFO) << "val_vec.size()=" << val_vec.size();
      this->a_list = std::vector<int32_t>(this->num_n, 0);
      for (std::size_t i(0); i < this->a_list.size(); i++) {
        this->a_list.at(i) = std::stoi(val_vec[i]);
      }
    }
  }
};

}  // namespace

TEST(SolverCTest, Sample1) {
  InData in_data(std::vector<std::string>({
      "3 34",
      "1",
      "8 13 26",
  }));

  EXPECT_EQ(solve(in_data.num_n, in_data.num_l, in_data.num_k, in_data.a_list), 13);
}

TEST(SolverCTest, Sample2) {
  InData in_data(std::vector<std::string>({
      "7 45",
      "2",
      "7 11 16 20 28 34 38",
  }));

  EXPECT_EQ(solve(in_data.num_n, in_data.num_l, in_data.num_k, in_data.a_list), 12);
}

TEST(SolverCTest, Sample3) {
  InData in_data(std::vector<std::string>({
      "3 100",
      "1",
      "28 54 81",

  }));

  EXPECT_EQ(solve(in_data.num_n, in_data.num_l, in_data.num_k, in_data.a_list), 46);
}

TEST(SolverCTest, Sample4) {
  InData in_data(std::vector<std::string>({
      "3 100",
      "2",
      "28 54 81",

  }));

  EXPECT_EQ(solve(in_data.num_n, in_data.num_l, in_data.num_k, in_data.a_list), 26);
}

TEST(SolverCTest, Sample5) {
  InData in_data(std::vector<std::string>({
      "20 1000",
      "4",
      "51 69 102 127 233 295 350 388 417 466 469 523 553 587 720 739 801 855 926 954",
  }));

  EXPECT_EQ(solve(in_data.num_n, in_data.num_l, in_data.num_k, in_data.a_list), 170);
}

}  // namespace a

}  // namespace typical90

}  // namespace atcoder

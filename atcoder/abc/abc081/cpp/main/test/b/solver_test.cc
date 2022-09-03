#include "b/solver.h"

#include <memory>
#include <string>

#include "gtest/gtest.h"

// TEST(SolverBTest, Stream) {
// std::stringstream ss = "8, 12, 40";
// std::vector < int> a_vector{8, 12, 40 };
// EXPECT_EQ(solve(a_vector), 2);
// }

TEST(SolverBTest, Sample) {
  std::vector<int> a_vector{8, 12, 40};
  EXPECT_EQ(solve(a_vector), 2);
}

TEST(SolverBTest, Sample2) {
  std::vector<int> a_vector{5, 6, 8, 10};
  EXPECT_EQ(solve(a_vector), 0);
}

TEST(SolverBTest, Sample3) {
  std::vector<int> a_vector{382253568, 723152896, 37802240,
                            379425024, 404894720, 471526144};
  EXPECT_EQ(solve(a_vector), 8);
}

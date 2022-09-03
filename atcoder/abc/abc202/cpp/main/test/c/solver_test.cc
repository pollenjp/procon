#include "c/solver.h"

#include <cmath>

#include "gtest/gtest.h"

TEST(SolverCTest, Sample1) {
  std::vector<int> A{1, 2, 2};
  std::vector<int> B{3, 1, 2};
  std::vector<int> C{2, 3, 2};
  EXPECT_EQ(solve(A, B, C), 4);
}

TEST(SolverCTest, Sample2) {
  std::vector<int> A{1, 1, 1, 1};
  std::vector<int> B{1, 1, 1, 1};
  std::vector<int> C{1, 2, 3, 4};
  EXPECT_EQ(solve(A, B, C), 16);
}

TEST(SolverCTest, Sample3) {
  std::vector<int> A{2, 3, 3};
  std::vector<int> B{1, 3, 3};
  std::vector<int> C{1, 1, 1};
  EXPECT_EQ(solve(A, B, C), 0);
}

TEST(SolverCTest, MySample1) {
  std::vector<int> A(static_cast<int>(std::pow(10.0F, 5.0)), 1);
  std::vector<int> B(static_cast<int>(std::pow(10.0F, 5.0)), 1);
  std::vector<int> C(static_cast<int>(std::pow(10.0F, 5.0)), 1);
  EXPECT_EQ(solve(A, B, C),
            static_cast<long long>(std::pow(std::pow(10.0L, 5.0), 2.0)));
}

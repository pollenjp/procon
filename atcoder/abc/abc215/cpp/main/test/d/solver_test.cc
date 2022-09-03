#include "d/solver.h"

#include "gtest/gtest.h"

TEST(SolverCTest, Sample1) {
  EXPECT_EQ(solve(3, 12, std::vector<int>({6, 1, 5})),
            std::vector<int>({1, 7, 11}));
}

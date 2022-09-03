#include "c/solver.h"

#include <vector>

#include "gtest/gtest.h"

TEST(SolverCTest, Sample1) { EXPECT_EQ(solve(7, 3, {1, 2, 1, 2, 3, 3, 1}), 3); }

TEST(SolverCTest, Sample2) { EXPECT_EQ(solve(5, 3, {4, 4, 4, 4, 4}), 1); }

TEST(SolverCTest, Sample3) {
  EXPECT_EQ(solve(10, 6,
                  {304621362, 506696497, 304621362, 506696497, 834022578,
                   304621362, 414720753, 304621362, 304621362, 414720753}),
            4);
}

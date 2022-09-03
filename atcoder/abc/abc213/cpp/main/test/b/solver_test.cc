#include "b/solver.h"

#include "gtest/gtest.h"

TEST(SolverATest, Sample1) {
  EXPECT_EQ(solve({1, 123, 12345, 12, 1234, 123456}), 3);
}

TEST(SolverATest, Sample2) { EXPECT_EQ(solve({3, 1, 4, 15, 9}), 5); }

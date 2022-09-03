#include "b/solver.h"

#include "gtest/gtest.h"

TEST(SolverBTest, Sample1) {
  EXPECT_EQ(solve(static_cast<std::string>("0601889")), std::string("6881090"));
}

TEST(SolverBTest, Sample2) {
  EXPECT_EQ(solve(static_cast<std::string>("86910")), std::string("01698"));
}

TEST(SolverBTest, Sample3) {
  EXPECT_EQ(solve(static_cast<std::string>("01010")), std::string("01010"));
}

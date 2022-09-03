#include "a/solver.h"

#include "gtest/gtest.h"

TEST(SolverCTest, Sample1) { EXPECT_EQ(solve(4, 8, 10), 2); }

TEST(SolverCTest, Sample2) { EXPECT_EQ(solve(10, 3, 4), 4); }

TEST(SolverCTest, Sample3) { EXPECT_EQ(solve(1, 2, 3), 0); }

TEST(SolverCTest, Sample4) {
  EXPECT_EQ(solve(1000000000000000, 1, 1000000000000000), 999999999999999);
}

TEST(SolverCTest, CustomSample1) {
  EXPECT_EQ(
      solve(static_cast<long long>(1e+15), 1, static_cast<long long>(1e+15)),
      static_cast<long long>(1e+15) - 1);
}

TEST(SolverCTest, CustomSample2) {
  EXPECT_EQ(
      solve(static_cast<long long>(1e+15), static_cast<long long>(1e+15), 1),
      static_cast<long long>(1e+15) - 1);
}

TEST(SolverCTest, CustomSample4) { EXPECT_EQ(solve(1001, 301, 1), 200); }

TEST(SolverCTest, CustomSample5) { EXPECT_EQ(solve(1001, 301, 501), 450); }

TEST(SolverCTest, CustomSample6) { EXPECT_EQ(solve(300, 300, 300), 0); }

TEST(SolverCTest, CustomSample7) { EXPECT_EQ(solve(1, 6, 14), 3); }

load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library", "cc_test")

cc_library(
    name = "SolverB",
    srcs = [],
    hdrs = [
        "src/b/solver.h",
    ],
    includes = ["src"],
    deps = [],
)

cc_binary(
    name = "main_b",
    srcs = glob(
        include = [
            "src/main_b.cc",
        ],
        exclude = [],
    ),
    deps = [
        ":SolverB",
    ],
)

cc_test(
    name = "solver_test",
    timeout = "short",
    srcs = glob(
        include = [
            "test/*/solver_test.cc",
        ],
        exclude = [],
    ),
    deps = [
        ":SolverB",
        "@gtest",
        "@gtest//:gtest_main",
    ],
)

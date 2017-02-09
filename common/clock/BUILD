package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)

rust_library(
    name = "clock",
    srcs = glob(["src/**/*.rs"]),
)

rust_test(
    name = "clock_unit_test",
    size = "small",
    srcs = glob(["src/**/*.rs"]),
)

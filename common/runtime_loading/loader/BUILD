package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

licenses(["notice"])

rust_library(
    name = "loader",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
      "//cargo/vendor:libloading",
      "//cargo/vendor:libc",
      "//cargo/vendor:clap",
    ],
)

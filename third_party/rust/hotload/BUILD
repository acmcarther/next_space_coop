package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

licenses(["notice"])

rust_library(
    name = "hotload",
    deps = [
      "//third_party/cargo2bazel/libloading",
      "//third_party/cargo2bazel/libc",
    ],
    srcs = glob(["lib.rs", "src/**/*.rs"])
)

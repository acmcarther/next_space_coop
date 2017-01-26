package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

rust_library(
    name = "hotloader",
    srcs = [
        "src/lib.rs",
    ],
    deps = [
      "//third_party/cargo2bazel/clap",
      "//third_party/rust/hotload",
    ],
)

package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

rust_library(
    name = "pubsub",
    srcs = [
        "lib.rs",
    ],
    deps = [
      "//third_party/cargo2bazel/specs",
      "//third_party/cargo2bazel/itertools",
    ]
)

package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)

rust_library(
    name = "physics",
    srcs = [
        "src/lib.rs",
    ],
    deps = [
      "//third_party/cargo2bazel/nphysics3d",
    ]
)

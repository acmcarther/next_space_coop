package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
)

rust_binary(
    name = "launcher",
    srcs = [
        "src/main.rs",
    ],
    deps = [
      "//third_party/cargo2bazel/clap",
      "//third_party/rust/hotload",
    ],
)

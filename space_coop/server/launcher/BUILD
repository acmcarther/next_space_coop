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
      "//common/runtime_loading/loader",
      "//cargo/vendor:clap",
    ],
)

package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
)

rust_binary(
    name = "client",
    srcs = [
        "src/main.rs",
    ],
    deps = [
      "//common",
      "//third_party/cargo2bazel/clap",
    ]
)

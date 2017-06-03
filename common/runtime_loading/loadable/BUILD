package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

rust_library(
    name = "loadable",
    srcs = glob(["src/**/*.rs"]),
    deps = [
        "@protoc_gen_rust//:protobuf",
        "//cargo/vendor:clap",
        "//cargo/vendor:log",
        "//cargo/vendor:libc",
    ]
)

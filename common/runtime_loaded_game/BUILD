package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

rust_library(
    name = "runtime_loaded_game",
    srcs = glob(["src/**/*.rs"]),
    deps = [
        "@protoc_gen_rust//:protobuf",
        "//third_party/cargo2bazel/clap",
        "//third_party/cargo2bazel/log",
        "//third_party/cargo2bazel/libc",
    ]
)

package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_doc",
    "rust_doc_test",
)

rust_library(
    name = "component",
    srcs = [
        "lib.rs",
    ],
    deps = [
        "//third_party/cargo2bazel/capnp",
    ]
)

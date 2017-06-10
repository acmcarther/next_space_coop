package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("//tools/bazel/proto:rust.bzl", "rust_proto_library")

rust_binary(
    name = "toy",
    srcs = glob(["src/**/*.rs"]),
    deps = [
      "//third_party/cargo2bazel/itertools",
      "//third_party/cargo2bazel/log",
      "//third_party/cargo2bazel/ash",
      "//third_party/cargo2bazel/glfw",
    ]
)

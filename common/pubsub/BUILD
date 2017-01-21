package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cc_proto_compile")
load("//tools/bazel/proto:rust.bzl", "rust_proto_library")

rust_library(
    name = "pubsub",
    srcs = [
        "src/lib.rs",
    ],
    deps = [
      "//third_party/cargo2bazel/specs",
      "//third_party/cargo2bazel/itertools",
    ]
)

rust_proto_library(
    name = "pubsub_proto",
    protos = ["pubsub.proto"],
)

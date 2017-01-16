package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cc_proto_compile")
load("//proto:rust.bzl", "rust_proto_library")

rust_proto_library(
    name = "service_proto",
    protos = ["service.proto"],
    proto_deps = ["//common/pubsub:pubsub_proto"],
)

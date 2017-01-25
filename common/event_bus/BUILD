package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cc_proto_compile")
load("//tools/bazel/proto:rust.bzl", "rust_proto_library")

rust_proto_library(
    name = "event_bus_proto",
    protos = ["event_bus.proto"],
)

rust_library(
    name = "event_bus",
    srcs = glob(["src/**/*.rs"]),
    deps = [
      ":event_bus_proto",
    ]
)

package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cc_proto_compile")
load("//tools/bazel-ext/proto:rust.bzl", "rust_proto_library")

rust_proto_library(
    name = "state_proto",
    proto_dep_crates = [
        "network_proto",
        "game_proto",
    ],
    proto_deps = [
        "//space_coop/common:game_proto",
        "//space_coop/common:network_proto",
    ],
    protos = ["state.proto"],
)

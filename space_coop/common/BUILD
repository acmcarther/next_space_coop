package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cc_proto_compile")
load("//tools/bazel-ext/proto:rust.bzl", "rust_proto_library")

rust_proto_library(
    name = "network_proto",
    proto_dep_crates = [
        "game_proto",
    ],
    proto_deps = [
        ":game_proto",
    ],
    protos = ["network.proto"],
)

rust_proto_library(
    name = "game_proto",
    protos = ["game.proto"],
)

package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cc_proto_compile")
load("//proto:rust.bzl", "rust_proto_library")

rust_proto_library(
    name = "state_proto",
    protos = ["state.proto"],
)

rust_library(
    name = "core",
    srcs = glob(["src/**/*.rs"]),
    visibility = ["//server:__subpackages__"],
    deps = [
        ":state_proto",
    ]
)

package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_doc",
    "rust_doc_test",
)

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cc_proto_compile")
load("//proto:rust.bzl", "rust_proto_library")

rust_library(
    name = "common",
    srcs = [
        "src/lib.rs",
    ],
    deps = [":some_proto"],
)

cc_proto_compile(
    name = "test-cpp",
    protos = ["some.proto"]
)

rust_proto_library(
    name = "some_proto",
    protos = ["some.proto"],
)

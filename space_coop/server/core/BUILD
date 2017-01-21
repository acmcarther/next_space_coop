package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cc_proto_compile")
load("//tools/bazel/proto:rust.bzl", "rust_proto_library")

rust_proto_library(
    name = "state_proto",
    protos = ["state.proto"],
    proto_deps = ["//common/service:service_proto"],
    proto_dep_crates = ["service_proto"],
)

rust_library(
    name = "core",
    srcs = glob(["src/**/*.rs"]),
    visibility = ["//server:__subpackages__"],
    crate_type = "dylib",
    deps = [
        ":state_proto",
        "//common/service",
        "//common/service:service_proto",
        "//space_coop/server/service_registry",
        "//third_party/cargo2bazel/libc",
        "//third_party/cargo2bazel/lazy_static",
    ]
)

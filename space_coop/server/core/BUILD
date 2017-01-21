package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cc_proto_compile")
load("//tools/bazel/proto:rust.bzl", "rust_proto_library")

rust_library(
    name = "core",
    srcs = glob(["src/**/*.rs"]),
    crate_type = "dylib",
    deps = [
        "@protoc_gen_rust//:protobuf",
        "//common/state:state_proto",
        "//common/service",
        "//common/service:service_proto",
        "//third_party/cargo2bazel/log",
        "//third_party/cargo2bazel/fern",
        "//third_party/cargo2bazel/time",
        "//third_party/cargo2bazel/libc",
        "//third_party/cargo2bazel/clap",
        "//third_party/cargo2bazel/lazy_static",
    ]
)

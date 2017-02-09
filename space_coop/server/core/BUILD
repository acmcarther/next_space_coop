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
        "//space_coop/common:network_proto",
        "//network/lite",
        "//common/runtime_loading/loadable",
        "//space_coop/server/core/state:state_proto",
        "//third_party/cargo2bazel/log",
        "//third_party/cargo2bazel/itertools",
        "//third_party/cargo2bazel/gaffer_udp",
        "//third_party/cargo2bazel/fern",
        "//third_party/cargo2bazel/time",
        "//third_party/cargo2bazel/libc",
        "//third_party/cargo2bazel/clap",
        "//third_party/cargo2bazel/lazy_static",
    ]
)

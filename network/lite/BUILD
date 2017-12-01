package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)
load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cc_proto_compile")
load("//tools/bazel-ext/proto:rust.bzl", "rust_proto_library")

rust_library(
    name = "lite",
    srcs = glob(["src/**/*.rs"]),
    deps = [
        ":lite_proto",
        "//cargo/vendor:gaffer_udp",
        "//cargo/vendor:itertools",
        "//cargo/vendor:log",
        "//common/clock",
        "@protoc_gen_rust//:protobuf",
    ],
)

rust_test(
    name = "lite_tests",
    size = "small",
    srcs = glob(["test/**/*.rs"]),
    deps = [
        ":lite",
        ":lite_proto",
        "//cargo/vendor:fern",
        "//cargo/vendor:gaffer_udp",
        "//cargo/vendor:itertools",
        "//cargo/vendor:lazy_static",
        "//cargo/vendor:log",
        "//cargo/vendor:time",
        "//common/clock",
        "@protoc_gen_rust//:protobuf",
    ],
)

rust_proto_library(
    name = "lite_proto",
    protos = ["lite.proto"],
)

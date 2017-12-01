package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)
load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cc_proto_compile")
load("//tools/bazel-ext/proto:rust.bzl", "rust_proto_library")

rust_library(
    name = "core",
    srcs = glob(["src/**/*.rs"]),
    #crate_type = "dylib",
    deps = [
        "//cargo/vendor:ash",
        "//cargo/vendor:clap",
        "//cargo/vendor:fern",
        "//cargo/vendor:gaffer_udp",
        "//cargo/vendor:itertools",
        "//cargo/vendor:lazy_static",
        "//cargo/vendor:libc",
        "//cargo/vendor:log",
        "//cargo/vendor:rand",
        "//cargo/vendor:time",
        "//common/runtime_loading/loadable",
        "//network/lite",
        "//space_coop/client/core/state:state_proto",
        "//space_coop/common:game_proto",
        "//space_coop/common:network_proto",
        "@protoc_gen_rust//:protobuf",
    ],
)

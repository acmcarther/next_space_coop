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
        "//space_coop/common:game_proto",
        "//network/lite",
        "//common/runtime_loading/loadable",
        "//space_coop/server/core/state:state_proto",
        "//cargo/vendor:log",
        "//cargo/vendor:itertools",
        "//cargo/vendor:gaffer_udp",
        "//cargo/vendor:rand",
        "//cargo/vendor:fern",
        "//cargo/vendor:time",
        "//cargo/vendor:libc",
        "//cargo/vendor:clap",
        "//cargo/vendor:lazy_static",
    ]
)

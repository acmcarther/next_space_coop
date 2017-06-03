package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cc_proto_compile")
load("//tools/bazel/proto:rust.bzl", "rust_proto_library")

rust_library(
    name = "lite",
    srcs = glob(["src/**/*.rs"]),
    deps = [
      ":lite_proto",
      "@protoc_gen_rust//:protobuf",
      "//common/clock",
      "//cargo/vendor:gaffer_udp",
      "//cargo/vendor:itertools",
      "//cargo/vendor:log",
    ]
)

rust_test(
    name = "lite_tests",
    srcs = glob(["test/**/*.rs"]),
    size = "small",
    deps = [
      ":lite",
      ":lite_proto",
      "@protoc_gen_rust//:protobuf",
      "//common/clock",
      "//cargo/vendor:gaffer_udp",
      "//cargo/vendor:fern",
      "//cargo/vendor:time",
      "//cargo/vendor:lazy_static",
      "//cargo/vendor:log",
      "//cargo/vendor:itertools",
    ]
)

rust_proto_library(
    name = "lite_proto",
    protos = ["lite.proto"],
)

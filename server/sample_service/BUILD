package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cc_proto_compile")
load("//proto:rust.bzl", "rust_proto_library")

rust_proto_library(
    name = "sample_service_proto",
    protos = ["sample_service.proto"],
)

rust_library(
    name = "sample_service",
    srcs = glob(["src/**/*.rs"]),
    deps = [
      ":sample_service_proto",
      "//common/service:service_proto",
      "//common/service",
      "//third_party/cargo2bazel/specs",
      "@protoc_gen_rust//:protobuf",
    ]
)

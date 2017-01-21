package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

rust_library(
    name = "service_registry",
    srcs = glob(["src/**/*.rs"]),
    deps = [
      "//common/service:service_proto",
      "//common/service",
      "//third_party/cargo2bazel/specs",
      "//third_party/cargo2bazel/itertools",
      "//space_coop/server/sample_service",
    ]
)

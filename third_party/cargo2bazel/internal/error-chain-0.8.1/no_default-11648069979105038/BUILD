# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/error_chain"
#
# If your package is not available there, you must require it explicitly in Cargo.toml and rerun
# cargo2bazel
package(default_visibility = [
  "//third_party/cargo2bazel:__subpackages__",
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

# TODO: we don't actually know this
licenses(["notice"])

rust_library(
    name = "error_chain",
    deps = [],
    srcs = ["//third_party/cargo2bazel/internal/error-chain-0.8.1:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/strsim"
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
    name = "strsim",
    deps = [],
    srcs = ["//third_party/cargo2bazel/internal/strsim-0.6.0:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

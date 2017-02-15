# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/tempdir"
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
    name = "tempdir",
    deps = ["//third_party/cargo2bazel/internal/rand-0.3.15/default-11648069979105038:rand"],
    srcs = ["//third_party/cargo2bazel/internal/tempdir-0.3.5:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

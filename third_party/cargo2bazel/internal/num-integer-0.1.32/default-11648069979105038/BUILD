# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/num_integer"
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
    name = "num_integer",
    deps = ["//third_party/cargo2bazel/internal/num-traits-0.1.36/default-11648069979105038:num_traits"],
    srcs = ["//third_party/cargo2bazel/internal/num-integer-0.1.32:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

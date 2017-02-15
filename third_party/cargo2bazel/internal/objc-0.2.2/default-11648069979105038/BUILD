# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/objc"
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
    name = "objc",
    deps = ["//third_party/cargo2bazel/internal/malloc_buf-0.0.6/default-11648069979105038:malloc_buf"],
    srcs = ["//third_party/cargo2bazel/internal/objc-0.2.2:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

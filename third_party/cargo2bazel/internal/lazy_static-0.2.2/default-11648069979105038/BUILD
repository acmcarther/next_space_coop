# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/lazy_static"
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
    name = "lazy_static",
    deps = [],
    srcs = ["//third_party/cargo2bazel/internal/lazy_static-0.2.2:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

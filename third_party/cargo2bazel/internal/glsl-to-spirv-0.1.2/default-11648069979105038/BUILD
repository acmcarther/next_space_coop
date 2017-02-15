# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/glsl_to_spirv"
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
    name = "glsl_to_spirv",
    deps = ["//third_party/cargo2bazel/internal/tempdir-0.3.5/default-11648069979105038:tempdir"],
    srcs = ["//third_party/cargo2bazel/internal/glsl-to-spirv-0.1.2:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

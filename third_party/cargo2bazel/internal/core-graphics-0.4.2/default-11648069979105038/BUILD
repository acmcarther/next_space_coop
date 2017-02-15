# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/core_graphics"
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
    name = "core_graphics",
    deps = ["//third_party/cargo2bazel/internal/libc-0.2.18/default-11648069979105038:libc", 
            "//third_party/cargo2bazel/internal/serde-0.8.23/default-11648069979105038:serde", 
            "//third_party/cargo2bazel/internal/core-foundation-0.2.3/default-11648069979105038:core_foundation"],
    srcs = ["//third_party/cargo2bazel/internal/core-graphics-0.4.2:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

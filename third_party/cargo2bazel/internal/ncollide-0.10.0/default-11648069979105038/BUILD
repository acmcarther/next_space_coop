# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/ncollide"
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
    name = "ncollide",
    deps = ["//third_party/cargo2bazel/internal/ncollide_utils-0.5.0/default-11648069979105038:ncollide_utils", 
            "//third_party/cargo2bazel/internal/ncollide_procedural-0.5.0/default-11648069979105038:ncollide_procedural", 
            "//third_party/cargo2bazel/internal/ncollide_transformation-0.5.0/default-11648069979105038:ncollide_transformation", 
            "//third_party/cargo2bazel/internal/ncollide_pipeline-0.9.0/default-11648069979105038:ncollide_pipeline", 
            "//third_party/cargo2bazel/internal/ncollide_math-0.5.0/default-11648069979105038:ncollide_math", 
            "//third_party/cargo2bazel/internal/ncollide_geometry-0.2.0/default-11648069979105038:ncollide_geometry"],
    srcs = ["//third_party/cargo2bazel/internal/ncollide-0.10.0:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

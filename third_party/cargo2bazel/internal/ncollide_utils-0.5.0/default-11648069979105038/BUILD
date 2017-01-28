# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/ncollide_utils"
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
    name = "ncollide_utils",
    deps = ["//third_party/cargo2bazel/internal/rand-0.3.15/default-11648069979105038:rand", 
            "//third_party/cargo2bazel/internal/ncollide_math-0.5.0/default-11648069979105038:ncollide_math", 
            "//third_party/cargo2bazel/internal/rustc-serialize-0.3.22/default-11648069979105038:rustc_serialize", 
            "//third_party/cargo2bazel/internal/num-0.1.36/default-11648069979105038:num", 
            "//third_party/cargo2bazel/internal/nalgebra-0.9.0/default-11648069979105038:nalgebra"],
    srcs = ["//third_party/cargo2bazel/internal/ncollide_utils-0.5.0:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

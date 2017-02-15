# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/serde_json"
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
    name = "serde_json",
    deps = ["//third_party/cargo2bazel/internal/dtoa-0.4.1/default-11648069979105038:dtoa", 
            "//third_party/cargo2bazel/internal/serde-0.9.7/default-11648069979105038:serde", 
            "//third_party/cargo2bazel/internal/itoa-0.3.1/default-11648069979105038:itoa", 
            "//third_party/cargo2bazel/internal/num-traits-0.1.36/default-11648069979105038:num_traits"],
    srcs = ["//third_party/cargo2bazel/internal/serde_json-0.9.6:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

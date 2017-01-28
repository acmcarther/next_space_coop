# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/num_rational"
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
    name = "num_rational",
    deps = ["//third_party/cargo2bazel/internal/num-bigint-0.1.35/default-11648069979105038:num_bigint", 
            "//third_party/cargo2bazel/internal/num-traits-0.1.36/default-11648069979105038:num_traits", 
            "//third_party/cargo2bazel/internal/num-integer-0.1.32/default-11648069979105038:num_integer"],
    srcs = ["//third_party/cargo2bazel/internal/num-rational-0.1.35:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = ["bigint", "rustc-serialize"],
)

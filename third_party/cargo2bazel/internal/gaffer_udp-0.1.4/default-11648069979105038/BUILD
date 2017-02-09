# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/gaffer_udp"
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
    name = "gaffer_udp",
    deps = ["//third_party/cargo2bazel/internal/itertools-0.5.9/default-11648069979105038:itertools", 
            "//third_party/cargo2bazel/internal/mio-0.6.2/default-11648069979105038:mio", 
            "//third_party/cargo2bazel/internal/byteorder-1.0.0/default-11648069979105038:byteorder"],
    srcs = ["//third_party/cargo2bazel/internal/gaffer_udp-0.1.4:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

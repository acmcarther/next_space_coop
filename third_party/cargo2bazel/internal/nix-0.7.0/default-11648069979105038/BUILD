# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/nix"
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
    name = "nix",
    deps = ["//third_party/cargo2bazel/internal/libc-0.2.18/default-11648069979105038:libc", 
            "//third_party/cargo2bazel/internal/cfg-if-0.1.0/default-11648069979105038:cfg_if", 
            "//third_party/cargo2bazel/internal/void-1.0.2/default-11648069979105038:void", 
            "//third_party/cargo2bazel/internal/bitflags-0.4.0/default-11648069979105038:bitflags"],
    srcs = ["//third_party/cargo2bazel/internal/nix-0.7.0:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

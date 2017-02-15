# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/cocoa"
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
    name = "cocoa",
    deps = ["//third_party/cargo2bazel/internal/libc-0.2.18/default-11648069979105038:libc", 
            "//third_party/cargo2bazel/internal/objc-0.2.2/default-11648069979105038:objc", 
            "//third_party/cargo2bazel/internal/core-graphics-0.4.2/default-11648069979105038:core_graphics", 
            "//third_party/cargo2bazel/internal/bitflags-0.7.0/default-11648069979105038:bitflags", 
            "//third_party/cargo2bazel/internal/block-0.1.6/default-11648069979105038:block"],
    srcs = ["//third_party/cargo2bazel/internal/cocoa-0.5.2:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/x11_dl"
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
    name = "x11_dl",
    deps = ["//third_party/cargo2bazel/internal/libc-0.2.18/default-11648069979105038:libc", 
            "//third_party/cargo2bazel/internal/lazy_static-0.2.2/default-11648069979105038:lazy_static"],
    srcs = ["//third_party/cargo2bazel/internal/x11-dl-2.12.0:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

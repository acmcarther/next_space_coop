# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/libloading"
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
    name = "libloading",
    deps = ["//third_party/cargo2bazel/internal/winapi-0.2.8/default-11648069979105038:winapi", 
            "//third_party/cargo2bazel/internal/lazy_static-0.2.2/default-11648069979105038:lazy_static", 
            "//third_party/cargo2bazel/internal/kernel32-sys-0.2.2/default-11648069979105038:kernel32_sys"],
    srcs = ["//third_party/cargo2bazel/internal/libloading-0.3.2:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

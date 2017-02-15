# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/miow"
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
    name = "miow",
    deps = ["//third_party/cargo2bazel/internal/winapi-0.2.8/default-11648069979105038:winapi", 
            "//third_party/cargo2bazel/internal/kernel32-sys-0.2.2/default-11648069979105038:kernel32_sys", 
            "//third_party/cargo2bazel/internal/net2-0.2.26/no_default-11648069979105038:net2", 
            "//third_party/cargo2bazel/internal/ws2_32-sys-0.2.1/default-11648069979105038:ws2_32_sys"],
    srcs = ["//third_party/cargo2bazel/internal/miow-0.2.0:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

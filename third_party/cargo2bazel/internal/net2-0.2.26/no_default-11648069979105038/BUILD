# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/net2"
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
    name = "net2",
    deps = ["//third_party/cargo2bazel/internal/libc-0.2.20/default-11648069979105038:libc", 
            "//third_party/cargo2bazel/internal/cfg-if-0.1.0/default-11648069979105038:cfg_if", 
            "//third_party/cargo2bazel/internal/ws2_32-sys-0.2.1/default-11648069979105038:ws2_32_sys", 
            "//third_party/cargo2bazel/internal/kernel32-sys-0.2.2/default-11648069979105038:kernel32_sys", 
            "//third_party/cargo2bazel/internal/winapi-0.2.8/default-11648069979105038:winapi"],
    srcs = ["//third_party/cargo2bazel/internal/net2-0.2.26:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

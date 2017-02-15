# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/tempfile"
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
    name = "tempfile",
    deps = ["//third_party/cargo2bazel/internal/libc-0.2.18/default-11648069979105038:libc", 
            "//third_party/cargo2bazel/internal/kernel32-sys-0.2.2/default-11648069979105038:kernel32_sys", 
            "//third_party/cargo2bazel/internal/rand-0.3.15/default-11648069979105038:rand", 
            "//third_party/cargo2bazel/internal/winapi-0.2.8/default-11648069979105038:winapi"],
    srcs = ["//third_party/cargo2bazel/internal/tempfile-2.1.5:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

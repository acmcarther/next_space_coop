# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/env_logger"
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
    name = "env_logger",
    deps = ["//third_party/cargo2bazel/internal/regex-0.2.1/default-11648069979105038:regex", 
            "//third_party/cargo2bazel/internal/log-0.3.6/default-11648069979105038:log"],
    srcs = ["//third_party/cargo2bazel/internal/env_logger-0.4.0:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = ["regex"],
)

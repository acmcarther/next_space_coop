# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/pulse"
#
# If your package is not available there, you must require it explicitly in Cargo.toml and rerun
# cargo2bazel
package(default_visibility = [
  "//third_party/cargo2bazel:__subpackages__",
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_doc",
    "rust_doc_test",
)

# TODO: we don't actually know this
licenses(["notice"])

rust_library(
    name = "pulse",
    deps = [
      "//third_party/cargo2bazel/internal/atom-0.3.4:atom",
    "//third_party/cargo2bazel/internal/time-0.1.35:time"
    ],
    srcs = glob(["lib.rs", "src/**/*.rs"])
)

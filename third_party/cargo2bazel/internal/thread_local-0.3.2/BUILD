# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/thread_local"
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
    name = "thread_local",
    deps = [
      "//third_party/cargo2bazel/internal/thread-id-3.0.0:thread_id",
    "//third_party/cargo2bazel/internal/unreachable-0.1.1:unreachable"
    ],
    srcs = glob(["lib.rs", "src/**/*.rs"])
)

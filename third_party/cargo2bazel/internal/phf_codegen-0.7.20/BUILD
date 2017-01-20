# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/phf_codegen"
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
    name = "phf_codegen",
    deps = [
      "//third_party/cargo2bazel/internal/phf_generator-0.7.20:phf_generator",
    "//third_party/cargo2bazel/internal/phf_shared-0.7.20:phf_shared"
    ],
    srcs = glob(["lib.rs", "src/**/*.rs"])
)

# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/target_build_utils"
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
    name = "target_build_utils",
    deps = [
      "//third_party/cargo2bazel/internal/phf-0.7.20:phf",
    "//third_party/cargo2bazel/internal/phf_codegen-0.7.20:phf_codegen",
    "//third_party/cargo2bazel/internal/serde_json-0.8.6:serde_json"
    ],
    srcs = glob(["lib.rs", "src/**/*.rs"])
)

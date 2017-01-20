# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/ncollide_math"
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
    name = "ncollide_math",
    deps = [
      "//third_party/cargo2bazel/internal/nalgebra-0.9.0:nalgebra",
    "//third_party/cargo2bazel/internal/num-0.1.36:num",
    "//third_party/cargo2bazel/internal/rand-0.3.15:rand",
    "//third_party/cargo2bazel/internal/rustc-serialize-0.3.22:rustc_serialize"
    ],
    srcs = glob(["lib.rs", "src/**/*.rs"])
)

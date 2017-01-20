# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/term_size"
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
    name = "term_size",
    deps = [
      "//third_party/cargo2bazel/internal/kernel32-sys-0.2.2:kernel32_sys",
    "//third_party/cargo2bazel/internal/libc-0.2.18:libc",
    "//third_party/cargo2bazel/internal/winapi-0.2.8:winapi"
    ],
    srcs = glob(["lib.rs", "src/**/*.rs"])
)

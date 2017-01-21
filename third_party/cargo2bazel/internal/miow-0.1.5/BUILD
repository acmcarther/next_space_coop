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
    "rust_binary",
    "rust_test",
    "rust_doc",
    "rust_doc_test",
)

# TODO: we don't actually know this
licenses(["notice"])

rust_library(
    name = "miow",
    deps = [
      "//third_party/cargo2bazel/internal/kernel32-sys-0.2.2:kernel32_sys",
    "//third_party/cargo2bazel/internal/net2-0.2.26:net2",
    "//third_party/cargo2bazel/internal/winapi-0.2.8:winapi",
    "//third_party/cargo2bazel/internal/ws2_32-sys-0.2.1:ws2_32_sys"
    ],
    srcs = glob(["lib.rs", "src/**/*.rs"])
)

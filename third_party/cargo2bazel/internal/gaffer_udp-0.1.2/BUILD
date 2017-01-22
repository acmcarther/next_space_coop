# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/gaffer_udp"
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
    name = "gaffer_udp",
    deps = [
      "//third_party/cargo2bazel/internal/byteorder-0.5.3:byteorder",
    "//third_party/cargo2bazel/internal/itertools-0.4.19:itertools",
    "//third_party/cargo2bazel/internal/mio-0.5.1:mio"
    ],
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    rustc_flags = [
      "--cap-lints warn",
    ],
)

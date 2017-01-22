# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/clap"
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
    name = "clap",
    deps = [
      "//third_party/cargo2bazel/internal/ansi_term-0.9.0:ansi_term",
    "//third_party/cargo2bazel/internal/bitflags-0.7.0:bitflags",
    "//third_party/cargo2bazel/internal/libc-0.2.18:libc",
    "//third_party/cargo2bazel/internal/strsim-0.6.0:strsim",
    "//third_party/cargo2bazel/internal/term_size-0.2.1:term_size",
    "//third_party/cargo2bazel/internal/unicode-segmentation-1.0.1:unicode_segmentation",
    "//third_party/cargo2bazel/internal/unicode-width-0.1.4:unicode_width",
    "//third_party/cargo2bazel/internal/vec_map-0.6.0:vec_map"
    ],
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    rustc_flags = [
      "--cap-lints warn",
    ],
)

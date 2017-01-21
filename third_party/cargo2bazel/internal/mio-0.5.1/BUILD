# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/mio"
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
    name = "mio",
    deps = [
      "//third_party/cargo2bazel/internal/bytes-0.3.0:bytes",
    "//third_party/cargo2bazel/internal/libc-0.2.18:libc",
    "//third_party/cargo2bazel/internal/log-0.3.6:log",
    "//third_party/cargo2bazel/internal/miow-0.1.5:miow",
    "//third_party/cargo2bazel/internal/net2-0.2.26:net2",
    "//third_party/cargo2bazel/internal/nix-0.5.1:nix",
    "//third_party/cargo2bazel/internal/slab-0.1.3:slab",
    "//third_party/cargo2bazel/internal/time-0.1.35:time",
    "//third_party/cargo2bazel/internal/winapi-0.2.8:winapi"
    ],
    srcs = glob(["lib.rs", "src/**/*.rs"])
)

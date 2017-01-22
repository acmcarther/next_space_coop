# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/specs"
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
    name = "specs",
    deps = [
      "//third_party/cargo2bazel/internal/atom-0.3.4:atom",
    "//third_party/cargo2bazel/internal/fnv-1.0.5:fnv",
    "//third_party/cargo2bazel/internal/mopa-0.2.2:mopa",
    "//third_party/cargo2bazel/internal/pulse-0.5.3:pulse",
    "//third_party/cargo2bazel/internal/threadpool-1.3.2:threadpool",
    "//third_party/cargo2bazel/internal/tuple_utils-0.2.0:tuple_utils"
    ],
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    crate_features = ["parallel"],
    rustc_flags = [
      "--cap-lints warn",
    ],
)

# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/vulkano"
#
# If your package is not available there, you must require it explicitly in Cargo.toml and rerun
# cargo2bazel
package(default_visibility = [
  "//third_party/cargo2bazel:__subpackages__",
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

# TODO: we don't actually know this
licenses(["notice"])

rust_library(
    name = "vulkano",
    deps = ["//third_party/cargo2bazel/internal/fnv-1.0.5/default-11648069979105038:fnv", 
            "//third_party/cargo2bazel/internal/crossbeam-0.2.10/default-11648069979105038:crossbeam", 
            "//third_party/cargo2bazel/internal/vk-sys-0.2.2/default-11648069979105038:vk_sys", 
            "//third_party/cargo2bazel/internal/lazy_static-0.2.2/default-11648069979105038:lazy_static", 
            "//third_party/cargo2bazel/internal/smallvec-0.3.1/default-11648069979105038:smallvec", 
            "//third_party/cargo2bazel/internal/shared_library-0.1.5/default-11648069979105038:shared_library"],
    srcs = ["//third_party/cargo2bazel/internal/vulkano-0.3.2:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

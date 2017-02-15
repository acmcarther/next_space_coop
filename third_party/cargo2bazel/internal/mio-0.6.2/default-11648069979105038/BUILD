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
)

# TODO: we don't actually know this
licenses(["notice"])

rust_library(
    name = "mio",
    deps = ["//third_party/cargo2bazel/internal/log-0.3.6/default-11648069979105038:log", 
            "//third_party/cargo2bazel/internal/winapi-0.2.8/default-11648069979105038:winapi", 
            "//third_party/cargo2bazel/internal/lazycell-0.4.0/default-11648069979105038:lazycell", 
            "//third_party/cargo2bazel/internal/kernel32-sys-0.2.2/default-11648069979105038:kernel32_sys", 
            "//third_party/cargo2bazel/internal/slab-0.3.0/default-11648069979105038:slab", 
            "//third_party/cargo2bazel/internal/miow-0.1.5/default-11648069979105038:miow", 
            "//third_party/cargo2bazel/internal/libc-0.2.18/default-11648069979105038:libc", 
            "//third_party/cargo2bazel/internal/nix-0.7.0/default-11648069979105038:nix", 
            "//third_party/cargo2bazel/internal/net2-0.2.26/default-11648069979105038:net2"],
    srcs = ["//third_party/cargo2bazel/internal/mio-0.6.2:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

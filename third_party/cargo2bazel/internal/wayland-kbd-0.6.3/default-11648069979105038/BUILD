# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/wayland_kbd"
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
    name = "wayland_kbd",
    deps = ["//third_party/cargo2bazel/internal/memmap-0.4.0/default-11648069979105038:memmap", 
            "//third_party/cargo2bazel/internal/lazy_static-0.2.2/default-11648069979105038:lazy_static", 
            "//third_party/cargo2bazel/internal/wayland-client-0.7.8/default-11648069979105038:wayland_client", 
            "//third_party/cargo2bazel/internal/bitflags-0.7.0/default-11648069979105038:bitflags", 
            "//third_party/cargo2bazel/internal/dlib-0.3.1/default-11648069979105038:dlib"],
    srcs = ["//third_party/cargo2bazel/internal/wayland-kbd-0.6.3:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

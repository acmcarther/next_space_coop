# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/wayland_window"
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
    name = "wayland_window",
    deps = ["//third_party/cargo2bazel/internal/tempfile-2.1.5/default-11648069979105038:tempfile", 
            "//third_party/cargo2bazel/internal/wayland-client-0.7.8/default--2708985055440507630:wayland_client", 
            "//third_party/cargo2bazel/internal/byteorder-1.0.0/default-11648069979105038:byteorder"],
    srcs = ["//third_party/cargo2bazel/internal/wayland-window-0.4.4:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

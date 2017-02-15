# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/wayland_client"
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
    name = "wayland_client",
    deps = ["//third_party/cargo2bazel/internal/libc-0.2.18/default-11648069979105038:libc", 
            "//third_party/cargo2bazel/internal/wayland-sys-0.7.8/default-8488286794866531169:wayland_sys", 
            "//third_party/cargo2bazel/internal/bitflags-0.7.0/default-11648069979105038:bitflags"],
    srcs = ["//third_party/cargo2bazel/internal/wayland-client-0.7.8:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = ["dlopen", "egl", "cursor"],
)

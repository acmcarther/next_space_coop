# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/winit"
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
    name = "winit",
    deps = ["//third_party/cargo2bazel/internal/x11-dl-2.12.0/default-11648069979105038:x11_dl", 
            "//third_party/cargo2bazel/internal/wayland-kbd-0.6.3/default-11648069979105038:wayland_kbd", 
            "//third_party/cargo2bazel/internal/wayland-window-0.4.4/default-11648069979105038:wayland_window", 
            "//third_party/cargo2bazel/internal/libc-0.2.18/default-11648069979105038:libc", 
            "//third_party/cargo2bazel/internal/objc-0.2.2/default-11648069979105038:objc", 
            "//third_party/cargo2bazel/internal/user32-sys-0.1.2/default-11648069979105038:user32_sys", 
            "//third_party/cargo2bazel/internal/cocoa-0.5.2/default-11648069979105038:cocoa", 
            "//third_party/cargo2bazel/internal/kernel32-sys-0.2.2/default-11648069979105038:kernel32_sys", 
            "//third_party/cargo2bazel/internal/gdi32-sys-0.1.1/default-11648069979105038:gdi32_sys", 
            "//third_party/cargo2bazel/internal/lazy_static-0.2.2/default-11648069979105038:lazy_static", 
            "//third_party/cargo2bazel/internal/core-graphics-0.4.2/default-11648069979105038:core_graphics", 
            "//third_party/cargo2bazel/internal/android_glue-0.2.1/default-11648069979105038:android_glue", 
            "//third_party/cargo2bazel/internal/shared_library-0.1.5/default-11648069979105038:shared_library", 
            "//third_party/cargo2bazel/internal/winapi-0.2.8/default-11648069979105038:winapi", 
            "//third_party/cargo2bazel/internal/wayland-client-0.7.8/default--4678207551517026016:wayland_client", 
            "//third_party/cargo2bazel/internal/cgl-0.1.5/default-11648069979105038:cgl", 
            "//third_party/cargo2bazel/internal/shell32-sys-0.1.1/default-11648069979105038:shell32_sys", 
            "//third_party/cargo2bazel/internal/core-foundation-0.2.3/default-11648069979105038:core_foundation", 
            "//third_party/cargo2bazel/internal/dwmapi-sys-0.1.0/default-11648069979105038:dwmapi_sys"],
    srcs = ["//third_party/cargo2bazel/internal/winit-0.5.10:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

"""
cargo-raze generated details for winit-0.5.11.

Generated for:
platform_triple: x86_64-unknown-linux-gnu
platform_attrs:
[
    "debug_assertions",
    "target_arch: x86_64",
    "target_endian: little",
    "target_env: gnu",
    "target_family: unix",
    "target_feature: sse",
    "target_feature: sse2",
    "target_has_atomic: 16",
    "target_has_atomic: 32",
    "target_has_atomic: 64",
    "target_has_atomic: 8",
    "target_has_atomic: ptr",
    "target_os: linux",
    "target_pointer_width: 64",
    "target_thread_local",
    "target_vendor: unknown",
    "unix"
]

DO NOT MODIFY! Instead, update vendor/CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "winit",
        pkg_version = "0.5.11",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [
        struct(
            name = "wayland-kbd",
            version = "0.6.3",
        ),
        struct(
            name = "shared_library",
            version = "0.1.5",
        ),
        struct(
            name = "x11-dl",
            version = "2.14.0",
        ),
        struct(
            name = "wayland-window",
            version = "0.4.4",
        ),
        struct(
            name = "wayland-client",
            version = "0.7.8",
        ),
        struct(
            name = "libc",
            version = "0.2.24",
        ),
        struct(
            name = "lazy_static",
            version = "0.2.8",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "winit",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "cursor",
            kinds = [
                "example",
            ],
            path = "examples/cursor.rs",
        ),
        struct(
            name = "transparent",
            kinds = [
                "example",
            ],
            path = "examples/transparent.rs",
        ),
        struct(
            name = "min_max_size",
            kinds = [
                "example",
            ],
            path = "examples/min_max_size.rs",
        ),
        struct(
            name = "window",
            kinds = [
                "example",
            ],
            path = "examples/window.rs",
        ),
        struct(
            name = "grabbing",
            kinds = [
                "example",
            ],
            path = "examples/grabbing.rs",
        ),
        struct(
            name = "fullscreen",
            kinds = [
                "example",
            ],
            path = "examples/fullscreen.rs",
        ),
        struct(
            name = "multiwindow",
            kinds = [
                "example",
            ],
            path = "examples/multiwindow.rs",
        ),
        struct(
            name = "window_proxy_send",
            kinds = [
                "test",
            ],
            path = "tests/window_proxy_send.rs",
        ),
    ],
)

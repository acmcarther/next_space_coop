"""
cargo-raze generated details for wayland-client-0.7.8.

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
        pkg_name = "wayland-client",
        pkg_version = "0.7.8",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [
        struct(
            name = "bitflags",
            version = "0.7.0",
        ),
        struct(
            name = "wayland-sys",
            version = "0.7.8",
        ),
        struct(
            name = "libc",
            version = "0.2.24",
        ),
    ],
    build_dependencies = [
        struct(
            name = "wayland-scanner",
            version = "0.7.8",
        ),
    ],
    dev_dependencies = [],
    features = [
        "wayland-sys",
        "cursor",
        "default",
        "dlopen",
        "egl",
    ],
    targets = [
        struct(
            name = "wayland-client",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "build-script-build",
            kinds = [
                "custom-build",
            ],
            path = "build.rs",
        ),
        struct(
            name = "simple_client",
            kinds = [
                "example",
            ],
            path = "examples/simple_client.rs",
        ),
        struct(
            name = "simple_window",
            kinds = [
                "example",
            ],
            path = "examples/simple_window.rs",
        ),
    ],
)

"""
cargo-raze generated details for wayland-kbd-0.6.3.

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
        pkg_name = "wayland-kbd",
        pkg_version = "0.6.3",
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
            name = "memmap",
            version = "0.4.0",
        ),
        struct(
            name = "wayland-client",
            version = "0.7.8",
        ),
        struct(
            name = "dlib",
            version = "0.3.1",
        ),
        struct(
            name = "lazy_static",
            version = "0.2.8",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [
        struct(
            name = "wayland-client",
            version = "0.7.8",
        ),
    ],
    features = [],
    targets = [
        struct(
            name = "wayland-kbd",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "basic_input",
            kinds = [
                "example",
            ],
            path = "examples/basic_input.rs",
        ),
    ],
)

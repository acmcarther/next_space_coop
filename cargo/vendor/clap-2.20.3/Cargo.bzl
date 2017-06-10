"""
cargo-raze generated details for clap-2.20.3.

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

DO NOT MODIFY! Instead, add a CargoOverride.bzl mixin.
"""
description = struct(
    package = struct(
        pkg_name = "clap",
        pkg_version = "2.20.3",
    ),
    dependencies = [
        struct(
            name = "vec_map",
            version = "0.6.0",
        ),
        struct(
            name = "bitflags",
            version = "0.7.0",
        ),
        struct(
            name = "term_size",
            version = "0.2.2",
        ),
        struct(
            name = "unicode-width",
            version = "0.1.4",
        ),
        struct(
            name = "ansi_term",
            version = "0.9.0",
        ),
        struct(
            name = "unicode-segmentation",
            version = "1.1.0",
        ),
        struct(
            name = "strsim",
            version = "0.6.0",
        ),
        struct(
            name = "libc",
            version = "0.2.20",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [
        "strsim",
        "wrap_help",
        "ansi_term",
        "color",
        "default",
        "term_size",
        "suggestions",
        "libc",
    ],
    targets = [
        struct(
            name = "clap",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
    ],
)

"""
cargo-raze generated details for mio-0.6.4.

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
        pkg_name = "mio",
        pkg_version = "0.6.4",
    ),
    dependencies = [
        struct(
            name = "net2",
            version = "0.2.26",
        ),
        struct(
            name = "libc",
            version = "0.2.20",
        ),
        struct(
            name = "log",
            version = "0.3.6",
        ),
        struct(
            name = "slab",
            version = "0.3.0",
        ),
        struct(
            name = "lazycell",
            version = "0.4.0",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "mio",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "test",
            kinds = [
                "test",
            ],
            path = "test/mod.rs",
        ),
    ],
)

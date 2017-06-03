"""
cargo-raze generated details for ash-0.14.2.

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
        pkg_name = "ash",
        pkg_version = "0.14.2",
    ),
    dependencies = [
        struct(
            name = "lazy_static",
            version = "0.2.2",
        ),
        struct(
            name = "shared_library",
            version = "0.1.5",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "ash",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
    ],
)

"""
cargo-raze generated details for ncollide-0.10.0.

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
        pkg_name = "ncollide",
        pkg_version = "0.10.0",
    ),
    dependencies = [
        struct(
            name = "ncollide_geometry",
            version = "0.2.0",
        ),
        struct(
            name = "ncollide_procedural",
            version = "0.5.0",
        ),
        struct(
            name = "ncollide_transformation",
            version = "0.5.0",
        ),
        struct(
            name = "ncollide_pipeline",
            version = "0.9.0",
        ),
        struct(
            name = "ncollide_math",
            version = "0.5.0",
        ),
        struct(
            name = "ncollide_utils",
            version = "0.5.0",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "ncollide",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "all",
            kinds = [
                "bench",
            ],
            path = "benches/all.rs",
        ),
    ],
)
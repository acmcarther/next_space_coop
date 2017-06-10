"""
cargo-raze generated details for target_build_utils-0.3.0.

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
        pkg_name = "target_build_utils",
        pkg_version = "0.3.0",
    ),
    dependencies = [
        struct(
            name = "phf",
            version = "0.7.21",
        ),
        struct(
            name = "serde_json",
            version = "0.9.6",
        ),
    ],
    build_dependencies = [
        struct(
            name = "phf_codegen",
            version = "0.7.21",
        ),
    ],
    dev_dependencies = [],
    features = [
        "default",
        "serde_json",
    ],
    targets = [
        struct(
            name = "target_build_utils",
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
    ],
)

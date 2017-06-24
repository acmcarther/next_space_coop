"""
cargo-raze generated details for ncollide_geometry-0.2.0.

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
        pkg_name = "ncollide_geometry",
        pkg_version = "0.2.0",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [
        struct(
            name = "nalgebra",
            version = "0.9.0",
        ),
        struct(
            name = "num",
            version = "0.1.39",
        ),
        struct(
            name = "ncollide_math",
            version = "0.5.0",
        ),
        struct(
            name = "rustc-serialize",
            version = "0.3.24",
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
            name = "ncollide_geometry",
            kinds = [
                "lib",
            ],
            path = "lib.rs",
        ),
    ],
)

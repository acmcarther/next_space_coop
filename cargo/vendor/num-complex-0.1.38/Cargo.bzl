"""
cargo-raze generated details for num-complex-0.1.38.

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
        pkg_name = "num-complex",
        pkg_version = "0.1.38",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [
        struct(
            name = "num-traits",
            version = "0.1.39",
        ),
        struct(
            name = "rustc-serialize",
            version = "0.3.24",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [
        "default",
        "rustc-serialize",
    ],
    targets = [
        struct(
            name = "num-complex",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
    ],
)

"""
cargo-raze generated details for bitflags-0.9.1.

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
        pkg_name = "bitflags",
        pkg_version = "0.9.1",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [],
    build_dependencies = [],
    dev_dependencies = [],
    features = [
        "example_generated",
        "default",
    ],
    targets = [
        struct(
            name = "bitflags",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "i128_bitflags",
            kinds = [
                "test",
            ],
            path = "tests/i128_bitflags.rs",
        ),
        struct(
            name = "external",
            kinds = [
                "test",
            ],
            path = "tests/external.rs",
        ),
        struct(
            name = "conflicting_trait_impls",
            kinds = [
                "test",
            ],
            path = "tests/conflicting_trait_impls.rs",
        ),
        struct(
            name = "external_no_std",
            kinds = [
                "test",
            ],
            path = "tests/external_no_std.rs",
        ),
    ],
)

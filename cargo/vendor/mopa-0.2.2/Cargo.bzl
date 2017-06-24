"""
cargo-raze generated details for mopa-0.2.2.

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
        pkg_name = "mopa",
        pkg_version = "0.2.2",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "mopa",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "no_std",
            kinds = [
                "example",
            ],
            path = "examples/no_std.rs",
        ),
        struct(
            name = "simple",
            kinds = [
                "example",
            ],
            path = "examples/simple.rs",
        ),
        struct(
            name = "no_std_or_alloc",
            kinds = [
                "example",
            ],
            path = "examples/no_std_or_alloc.rs",
        ),
    ],
)

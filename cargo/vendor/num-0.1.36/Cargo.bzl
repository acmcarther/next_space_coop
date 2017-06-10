"""
cargo-raze generated details for num-0.1.36.

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
        pkg_name = "num",
        pkg_version = "0.1.36",
    ),
    dependencies = [
        struct(
            name = "num-bigint",
            version = "0.1.35",
        ),
        struct(
            name = "num-rational",
            version = "0.1.35",
        ),
        struct(
            name = "num-traits",
            version = "0.1.36",
        ),
        struct(
            name = "num-iter",
            version = "0.1.32",
        ),
        struct(
            name = "num-integer",
            version = "0.1.32",
        ),
        struct(
            name = "num-complex",
            version = "0.1.35",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [
        "num-complex",
        "complex",
        "rustc-serialize",
        "default",
        "num-bigint",
        "bigint",
        "rational",
        "num-rational",
    ],
    targets = [
        struct(
            name = "num",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "bigint",
            kinds = [
                "bench",
            ],
            path = "benches/bigint.rs",
        ),
        struct(
            name = "shootout-pidigits",
            kinds = [
                "bench",
            ],
            path = "benches/shootout-pidigits.rs",
        ),
    ],
)

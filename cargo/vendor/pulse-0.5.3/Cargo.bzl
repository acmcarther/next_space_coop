"""
cargo-raze generated details for pulse-0.5.3.

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
        pkg_name = "pulse",
        pkg_version = "0.5.3",
    ),
    dependencies = [
        struct(
            name = "atom",
            version = "0.3.4",
        ),
        struct(
            name = "time",
            version = "0.1.36",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [
        "default",
    ],
    targets = [
        struct(
            name = "pulse",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "select",
            kinds = [
                "test",
            ],
            path = "tests/select.rs",
        ),
        struct(
            name = "simple",
            kinds = [
                "test",
            ],
            path = "tests/simple.rs",
        ),
        struct(
            name = "barrier",
            kinds = [
                "test",
            ],
            path = "tests/barrier.rs",
        ),
        struct(
            name = "benches",
            kinds = [
                "bench",
            ],
            path = "benches/benches.rs",
        ),
    ],
)

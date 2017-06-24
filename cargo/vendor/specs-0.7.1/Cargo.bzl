"""
cargo-raze generated details for specs-0.7.1.

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
        pkg_name = "specs",
        pkg_version = "0.7.1",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [
        struct(
            name = "fnv",
            version = "1.0.5",
        ),
        struct(
            name = "atom",
            version = "0.3.4",
        ),
        struct(
            name = "tuple_utils",
            version = "0.2.0",
        ),
        struct(
            name = "pulse",
            version = "0.5.3",
        ),
        struct(
            name = "threadpool",
            version = "1.3.2",
        ),
        struct(
            name = "mopa",
            version = "0.2.2",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [
        "threadpool",
        "parallel",
        "default",
        "pulse",
    ],
    targets = [
        struct(
            name = "specs",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "basic",
            kinds = [
                "example",
            ],
            path = "examples/basic.rs",
        ),
        struct(
            name = "tests",
            kinds = [
                "test",
            ],
            path = "tests/tests.rs",
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

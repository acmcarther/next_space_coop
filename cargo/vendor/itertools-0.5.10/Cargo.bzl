"""
cargo-raze generated details for itertools-0.5.10.

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
        pkg_name = "itertools",
        pkg_version = "0.5.10",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [
        struct(
            name = "either",
            version = "1.1.0",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "itertools",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "iris",
            kinds = [
                "example",
            ],
            path = "examples/iris.rs",
        ),
        struct(
            name = "zip",
            kinds = [
                "test",
            ],
            path = "tests/zip.rs",
        ),
        struct(
            name = "peeking_take_while",
            kinds = [
                "test",
            ],
            path = "tests/peeking_take_while.rs",
        ),
        struct(
            name = "tests",
            kinds = [
                "test",
            ],
            path = "tests/tests.rs",
        ),
        struct(
            name = "quick",
            kinds = [
                "test",
            ],
            path = "tests/quick.rs",
        ),
        struct(
            name = "tuples",
            kinds = [
                "test",
            ],
            path = "tests/tuples.rs",
        ),
        struct(
            name = "tuple_combinations",
            kinds = [
                "bench",
            ],
            path = "benches/tuple_combinations.rs",
        ),
        struct(
            name = "bench1",
            kinds = [
                "bench",
            ],
            path = "benches/bench1.rs",
        ),
        struct(
            name = "tuples",
            kinds = [
                "bench",
            ],
            path = "benches/tuples.rs",
        ),
    ],
)

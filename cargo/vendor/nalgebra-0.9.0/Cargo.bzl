"""
cargo-raze generated details for nalgebra-0.9.0.

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
        pkg_name = "nalgebra",
        pkg_version = "0.9.0",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [
        struct(
            name = "rustc-serialize",
            version = "0.3.24",
        ),
        struct(
            name = "num",
            version = "0.1.39",
        ),
        struct(
            name = "rand",
            version = "0.3.15",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "nalgebra",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "mat",
            kinds = [
                "test",
            ],
            path = "tests/mat.rs",
        ),
        struct(
            name = "arbitrary",
            kinds = [
                "test",
            ],
            path = "tests/arbitrary.rs",
        ),
        struct(
            name = "transforms",
            kinds = [
                "test",
            ],
            path = "tests/transforms.rs",
        ),
        struct(
            name = "op_assign",
            kinds = [
                "test",
            ],
            path = "tests/op_assign.rs",
        ),
        struct(
            name = "assert",
            kinds = [
                "test",
            ],
            path = "tests/assert.rs",
        ),
        struct(
            name = "vec",
            kinds = [
                "test",
            ],
            path = "tests/vec.rs",
        ),
        struct(
            name = "quat",
            kinds = [
                "test",
            ],
            path = "tests/quat.rs",
        ),
        struct(
            name = "mat",
            kinds = [
                "bench",
            ],
            path = "benches/mat.rs",
        ),
        struct(
            name = "construction",
            kinds = [
                "bench",
            ],
            path = "benches/construction.rs",
        ),
        struct(
            name = "vec",
            kinds = [
                "bench",
            ],
            path = "benches/vec.rs",
        ),
        struct(
            name = "dmat",
            kinds = [
                "bench",
            ],
            path = "benches/dmat.rs",
        ),
        struct(
            name = "quat",
            kinds = [
                "bench",
            ],
            path = "benches/quat.rs",
        ),
    ],
)

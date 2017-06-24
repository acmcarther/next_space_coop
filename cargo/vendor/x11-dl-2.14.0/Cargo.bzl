"""
cargo-raze generated details for x11-dl-2.14.0.

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
        pkg_name = "x11-dl",
        pkg_version = "2.14.0",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [
        struct(
            name = "libc",
            version = "0.2.24",
        ),
        struct(
            name = "lazy_static",
            version = "0.2.8",
        ),
    ],
    build_dependencies = [
        struct(
            name = "pkg-config",
            version = "0.3.9",
        ),
    ],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "x11-dl",
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
        struct(
            name = "hello-world",
            kinds = [
                "example",
            ],
            path = "examples/hello-world.rs",
        ),
    ],
)

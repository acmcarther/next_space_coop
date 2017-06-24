"""
cargo-raze generated details for cocoa-0.5.2.

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
        pkg_name = "cocoa",
        pkg_version = "0.5.2",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [
        struct(
            name = "block",
            version = "0.1.6",
        ),
        struct(
            name = "libc",
            version = "0.2.24",
        ),
        struct(
            name = "objc",
            version = "0.2.2",
        ),
        struct(
            name = "core-graphics",
            version = "0.4.2",
        ),
        struct(
            name = "bitflags",
            version = "0.7.0",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "cocoa",
            kinds = [
                "rlib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "tab_view",
            kinds = [
                "example",
            ],
            path = "examples/tab_view.rs",
        ),
        struct(
            name = "hello_world",
            kinds = [
                "example",
            ],
            path = "examples/hello_world.rs",
        ),
        struct(
            name = "foundation",
            kinds = [
                "test",
            ],
            path = "tests/foundation.rs",
        ),
    ],
)

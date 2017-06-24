"""
cargo-raze generated details for xml-rs-0.3.6.

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
        pkg_name = "xml-rs",
        pkg_version = "0.3.6",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [
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
            name = "xml",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "xml-analyze",
            kinds = [
                "bin",
            ],
            path = "src/analyze.rs",
        ),
        struct(
            name = "event_writer",
            kinds = [
                "test",
            ],
            path = "tests/event_writer.rs",
        ),
        struct(
            name = "event_reader",
            kinds = [
                "test",
            ],
            path = "tests/event_reader.rs",
        ),
    ],
)

"""
cargo-raze generated details for protobuf-1.2.1.

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
        pkg_name = "protobuf",
        pkg_version = "1.2.1",
    ),
    dependencies = [],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "protobuf",
            kinds = [
                "lib",
            ],
            path = "src/lib/lib.rs",
        ),
        struct(
            name = "protoc-gen-rust",
            kinds = [
                "bin",
            ],
            path = "src/protoc-gen-rust.rs",
        ),
        struct(
            name = "protobuf-bin-gen-rust-do-not-use",
            kinds = [
                "bin",
            ],
            path = "src/protobuf-bin-gen-rust-do-not-use.rs",
        ),
        struct(
            name = "coded_input_stream",
            kinds = [
                "bench",
            ],
            path = "benches/coded_input_stream.rs",
        ),
        struct(
            name = "coded_output_stream",
            kinds = [
                "bench",
            ],
            path = "benches/coded_output_stream.rs",
        ),
    ],
)

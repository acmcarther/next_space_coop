"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//cargo:__subpackages__"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)

# Unsupported target "coded_input_stream" with type "bench" omitted
# Unsupported target "coded_output_stream" with type "bench" omitted
rust_library(
    name = "protobuf",
    crate_root = "src/lib/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    crate_features = [
        
    ],
)
rust_binary(
    name = "protobuf_bin_gen_rust_do_not_use",
    crate_root = "src/protobuf-bin-gen-rust-do-not-use.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    crate_features = [
        
    ],
)
rust_binary(
    name = "protoc_gen_rust",
    crate_root = "src/protoc-gen-rust.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    crate_features = [
        
    ],
)

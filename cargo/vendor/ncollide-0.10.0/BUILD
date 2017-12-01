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

# Unsupported target "all" with type "bench" omitted
rust_library(
    name = "ncollide",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/ncollide_geometry-0.2.0:ncollide_geometry",
        "//cargo/vendor/ncollide_math-0.5.0:ncollide_math",
        "//cargo/vendor/ncollide_pipeline-0.9.0:ncollide_pipeline",
        "//cargo/vendor/ncollide_procedural-0.5.0:ncollide_procedural",
        "//cargo/vendor/ncollide_transformation-0.5.0:ncollide_transformation",
        "//cargo/vendor/ncollide_utils-0.5.0:ncollide_utils",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    crate_features = [
        
    ],
)

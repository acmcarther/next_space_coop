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

rust_library(
    name = "ncollide_math",
    crate_root = "lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/nalgebra-0.9.0:nalgebra",
        "//cargo/vendor/num-0.1.36:num",
        "//cargo/vendor/rand-0.3.15:rand",
        "//cargo/vendor/rustc-serialize-0.3.22:rustc_serialize",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    crate_features = [
        
    ],
)

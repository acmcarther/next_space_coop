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

# Unsupported target "arbitrary" with type "test" omitted
# Unsupported target "assert" with type "test" omitted
# Unsupported target "construction" with type "bench" omitted
# Unsupported target "dmat" with type "bench" omitted
# Unsupported target "mat" with type "bench" omitted
# Unsupported target "mat" with type "test" omitted
rust_library(
    name = "nalgebra",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
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
# Unsupported target "op_assign" with type "test" omitted
# Unsupported target "quat" with type "bench" omitted
# Unsupported target "quat" with type "test" omitted
# Unsupported target "transforms" with type "test" omitted
# Unsupported target "vec" with type "bench" omitted
# Unsupported target "vec" with type "test" omitted

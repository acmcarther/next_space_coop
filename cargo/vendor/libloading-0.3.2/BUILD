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

# Unsupported target "functions" with type "test" omitted
rust_library(
    name = "libloading",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/lazy_static-0.2.2:lazy_static",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    crate_features = [
        
    ],
)
# Unsupported target "markers" with type "test" omitted
# Unsupported target "statics" with type "test" omitted
# Unsupported target "windows" with type "test" omitted

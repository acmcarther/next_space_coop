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

# Unsupported target "basic" with type "example" omitted
# Unsupported target "benches" with type "bench" omitted
rust_library(
    name = "specs",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/atom-0.3.4:atom",
        "//cargo/vendor/fnv-1.0.5:fnv",
        "//cargo/vendor/mopa-0.2.2:mopa",
        "//cargo/vendor/pulse-0.5.3:pulse",
        "//cargo/vendor/threadpool-1.3.2:threadpool",
        "//cargo/vendor/tuple_utils-0.2.0:tuple_utils",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    crate_features = [
        "default",
        "parallel",
        "pulse",
        "threadpool",
    ],
)
# Unsupported target "tests" with type "test" omitted

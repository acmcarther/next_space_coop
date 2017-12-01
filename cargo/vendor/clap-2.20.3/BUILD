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
    name = "clap",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/ansi_term-0.9.0:ansi_term",
        "//cargo/vendor/bitflags-0.7.0:bitflags",
        "//cargo/vendor/libc-0.2.20:libc",
        "//cargo/vendor/strsim-0.6.0:strsim",
        "//cargo/vendor/term_size-0.2.2:term_size",
        "//cargo/vendor/unicode-segmentation-1.1.0:unicode_segmentation",
        "//cargo/vendor/unicode-width-0.1.4:unicode_width",
        "//cargo/vendor/vec_map-0.6.0:vec_map",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    crate_features = [
        "ansi_term",
        "color",
        "default",
        "libc",
        "strsim",
        "suggestions",
        "term_size",
        "wrap_help",
    ],
)

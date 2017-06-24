"""
cargo-raze direct Cargo.toml dependencies.

This BUILD file provides aliases to explicit cargo dependencies and is
the only way to access vendored dependencies.

If a dependency is missing, add it as an explicit root dependency in
Cargo.toml and rerun raze.

This file is overridden on runs of raze; do not add anything to it.

If that is causing you pain, please drop a line in acmcarther/cargo-raze.
"""
package(default_visibility = ["//visibility:public"])

alias(
    name = "ash",
    actual = "//cargo/vendor/ash-0.14.3:ash",
)

alias(
    name = "capnp",
    actual = "//cargo/vendor/capnp-0.7.5:capnp",
)

alias(
    name = "clap",
    actual = "//cargo/vendor/clap-2.25.0:clap",
)

alias(
    name = "env_logger",
    actual = "//cargo/vendor/env_logger-0.4.3:env_logger",
)

alias(
    name = "error-chain",
    actual = "//cargo/vendor/error-chain-0.8.1:error_chain",
)

alias(
    name = "fern",
    actual = "//cargo/vendor/fern-0.3.5:fern",
)

alias(
    name = "gaffer_udp",
    actual = "//cargo/vendor/gaffer_udp-0.1.4:gaffer_udp",
)

alias(
    name = "itertools",
    actual = "//cargo/vendor/itertools-0.5.10:itertools",
)

alias(
    name = "lazy_static",
    actual = "//cargo/vendor/lazy_static-0.2.8:lazy_static",
)

alias(
    name = "libc",
    actual = "//cargo/vendor/libc-0.2.24:libc",
)

alias(
    name = "libloading",
    actual = "//cargo/vendor/libloading-0.3.4:libloading",
)

alias(
    name = "log",
    actual = "//cargo/vendor/log-0.3.8:log",
)

alias(
    name = "nphysics3d",
    actual = "//cargo/vendor/nphysics3d-0.5.0:nphysics3d",
)

alias(
    name = "openssl",
    actual = "//cargo/vendor/openssl-0.9.14:openssl",
)

alias(
    name = "protobuf",
    actual = "//cargo/vendor/protobuf-1.3.1:protobuf",
)

alias(
    name = "rand",
    actual = "//cargo/vendor/rand-0.3.15:rand",
)

alias(
    name = "specs",
    actual = "//cargo/vendor/specs-0.7.1:specs",
)

alias(
    name = "time",
    actual = "//cargo/vendor/time-0.1.37:time",
)

alias(
    name = "winit",
    actual = "//cargo/vendor/winit-0.5.11:winit",
)


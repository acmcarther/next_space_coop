# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/backtrace"
#
# If your package is not available there, you must require it explicitly in Cargo.toml and rerun
# cargo2bazel
package(default_visibility = [
  "//third_party/cargo2bazel:__subpackages__",
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

# TODO: we don't actually know this
licenses(["notice"])

rust_library(
    name = "backtrace",
    deps = ["//third_party/cargo2bazel/internal/cfg-if-0.1.0/default-11648069979105038:cfg_if", 
            "//third_party/cargo2bazel/internal/libc-0.2.18/default-11648069979105038:libc", 
            "//third_party/cargo2bazel/internal/rustc-demangle-0.1.3/default-11648069979105038:rustc_demangle", 
            "//third_party/cargo2bazel/internal/kernel32-sys-0.2.2/default-11648069979105038:kernel32_sys", 
            "//third_party/cargo2bazel/internal/backtrace-sys-0.1.5/default-11648069979105038:backtrace_sys", 
            "//third_party/cargo2bazel/internal/dbghelp-sys-0.2.0/default-11648069979105038:dbghelp_sys", 
            "//third_party/cargo2bazel/internal/winapi-0.2.8/default-11648069979105038:winapi"],
    srcs = ["//third_party/cargo2bazel/internal/backtrace-0.3.0:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = ["libunwind", "libbacktrace", "coresymbolication", "dladdr", "dbghelp"],
)

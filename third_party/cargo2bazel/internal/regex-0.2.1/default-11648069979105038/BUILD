# This is (probably) a transitive dependency
# You likely want "//third_party/cargo2bazel/regex"
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
    name = "regex",
    deps = ["//third_party/cargo2bazel/internal/utf8-ranges-1.0.0/default-11648069979105038:utf8_ranges", 
            "//third_party/cargo2bazel/internal/regex-syntax-0.4.0/default-11648069979105038:regex_syntax", 
            "//third_party/cargo2bazel/internal/thread_local-0.3.2/default-11648069979105038:thread_local", 
            "//third_party/cargo2bazel/internal/aho-corasick-0.6.2/default-11648069979105038:aho_corasick", 
            "//third_party/cargo2bazel/internal/memchr-1.0.1/default-11648069979105038:memchr"],
    srcs = ["//third_party/cargo2bazel/internal/regex-0.2.1:sources"],
    rustc_flags = [
      "--cap-lints warn",
    ],
    crate_features = [],
)

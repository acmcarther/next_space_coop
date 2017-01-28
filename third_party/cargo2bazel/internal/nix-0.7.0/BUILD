# This just globs the rust sources so that they can be accessed by inner dirs
package(default_visibility = [
  "//third_party/cargo2bazel:__subpackages__",
])

# TODO: we don't actually know this
licenses(["notice"])

filegroup(
    name = "sources",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
)

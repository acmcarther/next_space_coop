"""
cargo-raze details override for num-0.1.36.

Make your changes here. Bazel automatically integrates overrides from this
file and will not overwrite it on a rerun of cargo-raze.

Environment variables should be of the form

("key", "value")

Dependencies should be of the form

struct(
    name = "some-dependency",
    path = "//some/depot/path",
)
"""
override = struct(
  rustc_flags = [],
  environment_variables = [],
  dependencies = []
)

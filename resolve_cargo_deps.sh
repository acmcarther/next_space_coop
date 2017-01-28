#!/usr/bin/env bash
cargo fetch
bazel run @io_bazel_cargo2bazel//cargo2bazel $(pwd)/Cargo.toml $(pwd)/Cargo.lock $(pwd) --verbose_failures --action_env=PATH="$PATH" --action_env=LD_LIBRARY_PATH="$LD_LIBRARY_PATH"


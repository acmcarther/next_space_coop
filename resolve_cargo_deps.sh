#!/usr/bin/env bash
cargo fetch
bazel run @io_bazel_cargo2bazel//cargo2bazel $(pwd)/Cargo.lock $(pwd)

#!/usr/bin/env bash
bazel run @io_bazel_cargo2bazel//cargo2bazel $(pwd)/Cargo.lock $(pwd)

next space coop
===============

# Setup
Make sure you have a (recent) installation of bazel.

## Pulling cargo dependencies down

Then, inspect and run `resolve_cargo_deps.sh`. This file will parse the Cargo.lock and generate bazel packages for all of your direct and indirect dependencies. These dependencies end up living in `//third_party/cargo2bazel`, but they're not included in the repository at this time.

## Working with Cargo.toml

We use Cargo (Rust's package manager) for dependency location and resolution, but there is a bit of a ritual to perform to acquire new dependencies.

1. Add a dependency to the Cargo.toml
2. Execute `cargo fetch` to resolve the dependencies to the `Cargo.lock`
3. Re-run `resolve_cargo_deps.sh` to rebuild the list of dependencies.


## Building Rust on Nix

For users on Nixos, there is an extra step -- patching the rustc the cargo pulls down. You'll want to run something like the following (adjusted for your user and bazel path).

```
 patchelf  --set-interpreter "$(cat $NIX_CC/nix-support/dynamic-linker)" ~/.cache/bazel/_bazel_alex/137a6182ca2a00cee867ee137dbd907b/execroot/prototype3/external/rust_linux_x86_64/rustc/bin/rustc
```

## Running the client or the server

Finally, running either the client or the server is as simple as

```
bazel run //client
```

and

```
bazel run //server
```

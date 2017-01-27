# next space coop

## Setup
Make sure you have a (recent) installation of bazel.

## Working on next_space_coop

### Pulling cargo dependencies down

All required third party dependencies should already exist in //third_party.


### Working with Cargo.toml

We use Cargo (Rust's package manager) for dependency location and resolution, but there is a bit of a ritual to perform to acquire new dependencies.

1. Add a dependency to the Cargo.toml
2. Execute `cargo fetch` to resolve the dependencies to the `Cargo.lock`
3. Re-run `resolve_cargo_deps.sh` to rebuild the list of dependencies.

Theres currently a small set of edits to the generated BUILD files to handle some corner cases, which need to be reapplied after regenerating the cargo dependencies

### Building Rust on Nix

For users on Nixos, there is an extra step -- patching the rustc the cargo pulls down. You'll want to run something like the following (adjusted for your user and bazel path).

```
 patchelf  --set-interpreter "$(cat $NIX_CC/nix-support/dynamic-linker)" ~/.cache/bazel/_bazel_alex/137a6182ca2a00cee867ee137dbd907b/execroot/prototype3/external/rust_linux_x86_64/rustc/bin/rustc
```

## Code organization

See the root directories for information on their purposes

## Running the application

In the case of space_coop (the testbed game), for the client and the server, you'll need to build the `core` crate, which is the hotloaded game, and the `launcher` crate, which hotloads the game. My current workflow is as follows.

```bash
// In one window
//
// Start autobuilding server core
./space_coop/server/core/hotbuild.sh

// In another window
bazel build //space_coop/server/launcher
// Launch outside bazel to avoid blocking rebuilds of core
./space_coop/server/launcher
```

This workflow keeps the latest code loaded at all times, and automatically compiles for you similar to `cargo watch`.

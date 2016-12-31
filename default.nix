# TODO: Untangle this crazy business
with (import <nixpkgs> {});
let
  pkgs = import <nixpkgs> {};
in pkgs.stdenv.mkDerivation rec {
  name = "space_coop";
  shellHook = ''
    # Allow my shell to add custom snippet
    export IS_NIX_SHELL=1
    export BAZEL_SH=/run/current-system/sw/bin/bash
  '';
}

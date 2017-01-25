# TODO: Untangle this crazy business
with (import <nixpkgs> {});
let
  pkgs = import <nixpkgs> {};
in pkgs.stdenv.mkDerivation rec {
  name = "space_coop";
  buildInputs = [ pkgs.inotify-tools ];
  LD_LIBRARY_PATH = "${stdenv.lib.makeLibraryPath [ stdenv.cc.cc.lib stdenv.glibc.out xorg.libX11 xorg.libXcursor gtk2 glib curl "$out" ]}:${stdenv.cc.cc.lib}/lib64:${stdenv.glibc.out}/lib64";
  shellHook = ''
    # Allow my shell to add custom snippet
    export IS_NIX_SHELL=1
    export BAZEL_SH=/run/current-system/sw/bin/bash
  '';
}

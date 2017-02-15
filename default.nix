# TODO: Untangle this crazy business
with (import <nixpkgs> {});
let
  pkgs = import <nixpkgs> {};
  bazel-custom = import ./nix/bazel/default.nix {
    inherit (pkgs) stdenv fetchFromGitHub buildFHSUserEnv writeScript jdk zip unzip;
    inherit (pkgs) which makeWrapper binutils fetchurl;
  };
in pkgs.stdenv.mkDerivation rec {
  name = "space_coop";
  buildInputs = [ pkgs.vulkan-loader pkgs.inotify-tools bazel-custom pkgs.bash pkgs.patchelf ];
  LD_LIBRARY_PATH = "${stdenv.lib.makeLibraryPath [ stdenv.cc.cc.lib stdenv.glibc.out xorg.libX11 xorg.libXcursor gtk2 glib curl "$out" ]}:${stdenv.cc.cc.lib}/lib64:${stdenv.glibc.out}/lib64";
  shellHook = ''
    # Allow my shell to add custom snippet
    export IS_NIX_SHELL=1
    export BAZEL_SH=/run/current-system/sw/bin/bash
  '';
}

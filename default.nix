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
  buildInputs = with pkgs; [
    vulkan-loader
    inotify-tools
    bazel-custom
    glfw3
    bash
    patchelf
    freetype
    mesa
    cmake
    xlibs.libX11
    xlibs.libXxf86vm
    xlibs.libXi
    xlibs.libXrandr
    xlibs.libXinerama
    xlibs.libXcursor
    xlibs.libXext
    xlibs.libXfixes
  ];
  shellHook = with pkgs.xlibs; ''
    # Allow my shell to add custom snippet
    export IS_NIX_SHELL=1
    export BAZEL_SH=/run/current-system/sw/bin/bash
    LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${stdenv.lib.makeLibraryPath [ stdenv.cc.cc.lib stdenv.glibc.out xorg.libX11 xorg.libX11.dev xorg.libXcursor gtk2 glib curl "$out" ]}:${stdenv.cc.cc.lib}/lib64:${stdenv.glibc.out}/lib64:${pkgs.mesa}/lib:${libX11}/lib:${libXcursor}/lib:${libXxf86vm}/lib:${libXi}/lib:${pkgs.mesa_glu}/lib:${xlibsWrapper}/lib:${pkgs.freeglut}/lib:${libXext}/lib:${pkgs.glfw3}/lib:${pkgs.vtk}/lib:${libXrandr}/lib:${libXfixes}/lib:${libXinerama}/lib:${libXcursor}/lib:{${pkgs.mesa_drivers}/lib:${pkgs.vulkan-loader}/lib";
  '';
}

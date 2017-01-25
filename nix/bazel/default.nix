{ stdenv, fetchFromGitHub, fetchurl, buildFHSUserEnv, writeScript, jdk, zip, unzip,
  which, makeWrapper, binutils }:

let

  version = "0.4.3";

  meta = with stdenv.lib; {
    homepage = http://github.com/bazelbuild/bazel/;
    description = "Build tool that builds code quickly and reliably";
    license = licenses.asl20;
    maintainers = [ maintainers.philandstuff ];
    platforms = platforms.linux;
  };

  bootstrapEnv = buildFHSUserEnv {
    name = "bazel-bootstrap-env";

    targetPkgs = pkgs: [ ];

    inherit meta;
  };

  bazelBinary = stdenv.mkDerivation rec {
    name = "bazel-${version}";

    src = fetchurl {
      url = "https://github.com/bazelbuild/bazel/releases/download/0.4.3/bazel-0.4.3-dist.zip";
      sha256 = "03zrch5m4kagzssxa75hrabjvvfdbf1bzchqrwbp7hc105caplnb";
    };
    sourceRoot = ".";
    patches = [ ./java_stub_template.patch ./allow_overriding_sh.patch ];

    packagesNotFromEnv = [
        stdenv.cc stdenv.cc.cc.lib jdk which zip unzip binutils ];
    buildInputs = packagesNotFromEnv ++ [ bootstrapEnv makeWrapper ];

    buildTimeBinPath = stdenv.lib.makeBinPath packagesNotFromEnv;
    buildTimeLibPath = stdenv.lib.makeLibraryPath packagesNotFromEnv;

    runTimeBinPath = stdenv.lib.makeBinPath [ jdk stdenv.cc.cc ];
    runTimeLibPath = stdenv.lib.makeLibraryPath [ stdenv.cc.cc.lib ];

    buildWrapper = writeScript "build-wrapper.sh" ''
      #! ${stdenv.shell} -e
      export PATH="${buildTimeBinPath}:$PATH"
      export LD_LIBRARY_PATH="${buildTimeLibPath}:$LD_LIBRARY_PATH"
      ./compile.sh
    '';

    buildPhase = ''
      bazel-bootstrap-env ${buildWrapper}
    '';

    installPhase = ''
      mkdir -p $out/bin
      cp output/bazel $out/bin/
      wrapProgram $out/bin/bazel \
          --suffix PATH ":" "${runTimeBinPath}" \
          --suffix LD_LIBRARY_PATH ":" "${runTimeLibPath}"
    '';

    dontStrip = true;
    dontPatchELF = true;

    inherit meta;
  };

in bazelBinary

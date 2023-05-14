{ stable ? import <nixpkgs> { }, ... }:
stable.mkShell rec {
  developmentInputs = (with stable; [ ]);
  buildInputs =
    (with stable; [
      rustup
      gcc
      pkg-config
      lldb
      cargo
      rustc
      clippy
      openssl
      freetype
      expat
      llvmPackages.bintools
      git-lfs
      git-cliff
      rust-analyzer
      cargo-hack
      cargo-watch
      just
    ]);

  RUST_SRC_PATH = "${stable.rust.packages.stable.rustPlatform.rustLibSrc}";
  shellHook = ''
    export PATH="$CARGO_HOME:$PATH";
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${stable.lib.makeLibraryPath buildInputs}";
  '';
}

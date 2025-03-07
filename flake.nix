{
  description = "CoKoala project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };
        toolchain = fenix.packages.${system}.complete.withComponents [
          "llvm-tools-preview"
          "rust-analyzer"
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            broot
            git
            cargo-leptos
            cargo-generate
            leptosfmt
            openssl
            pkg-config
            tailwindcss
            tailwindcss-language-server
            trunk
            wasm-bindgen-cli
            toolchain
          ];

          shellHook = ''
            if command -v nu >/dev/null 2>&1; then
              exec nu
            else
              echo "nu command not found, skipping shellHook"
            fi
          '';

          RUST_LOG = "debug";
          RUST_BACKTRACE = "1";
          RUSTFLAGS = "--cfg=web_sys_unstable_apis";
        };

        packages.default = pkgs.stdenv.mkDerivation {
          name = "cokoala";
          src = ./.;
          nativeBuildInputs = [
            (fenix.packages.${system}.complete.withComponents [
              "cargo"
              "rustc"
              "rust-src"
              "rustfmt"
              "clippy"
              "llvm-tools-preview"
            ])
            pkgs.trunk
            pkgs.wasm-bindgen-cli
            pkgs.binaryen
          ];
          buildInputs = with pkgs; [
            tailwindcss
            bun
            openssl
            pkg-config
          ];
          buildPhase = ''
            export HOME=$TMPDIR
            export RUSTFLAGS="-C target-feature=+crt-static"
            export CARGO_TARGET_DIR=$TMPDIR/target
            export CARGO_HOME=$TMPDIR/.cargo
            mkdir -p $CARGO_HOME
            cat <<EOF > $CARGO_HOME/config.toml
            [build]
            target = "wasm32-unknown-unknown"
            EOF
            trunk build --release
          '';
          installPhase = ''
            mkdir -p $out
            cp -r dist/* $out/
          '';
        };
      }
    );
}

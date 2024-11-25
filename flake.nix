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
        pkgs = nixpkgs.legacyPackages.${system};

        rustToolchain =
          with fenix.packages.${system};
          combine [
            latest.toolchain
            targets.wasm32-unknown-unknown.latest.rust-std
          ];

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            binaryen
            git
            cargo-leptos
            leptosfmt
            openssl
            pkg-config
            rustToolchain
            trunk
            wasm-bindgen-cli
            spin
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
        };
      }
    );
}

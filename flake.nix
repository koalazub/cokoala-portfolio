{
  description = "CoKoala project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShells = {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              binaryen
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
        };
      }
    );
}

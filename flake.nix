{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      overlays = [
        (import rust-overlay)
        (self: super: {
          rustToolchain = super.rust-bin.selectLatestNightlyWith (
            toolchain:
            toolchain.default.override {
              targets = [
                "x86_64-unknown-linux-gnu"
                "wasm32-unknown-unknown"
              ];
              extensions = [
                "rustc-codegen-cranelift-preview"
                "rust-src"
                "rustfmt"
              ];
            }
          );
        })
      ];

      forAllSystems =
        function:
        nixpkgs.lib.genAttrs systems (
          system:
          let
            pkgs = import nixpkgs { inherit system overlays; };
          in
          function pkgs
        );
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell (
          pkgs.lib.fix (finalAttrs: {
            buildInputs = builtins.attrValues {
              inherit (pkgs)
                rustToolchain
                rust-analyzer-unwrapped
                dioxus-cli
                nixd
                wasm-bindgen-cli
                tailwindcss
                tailwindcss-language-server
                vscode-css-languageserver
                ;
            };
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath finalAttrs.buildInputs;
            RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
          })
        );
      });

      formatter = forAllSystems (
        pkgs:
        pkgs.writeShellApplication {
          name = "nix3-fmt-wrapper";

          runtimeInputs = builtins.attrValues {
            inherit (pkgs)
              rustToolchain
              nixfmt-rfc-style
              taplo
              fd
              dioxus-cli
              ;
          };

          text = ''
            fd "$@" -t f -e nix -x nixfmt -q '{}'
            fd "$@" -t f -e toml -x taplo format '{}'
            dx fmt
            cargo fmt
          '';
        }
      );
    };
}

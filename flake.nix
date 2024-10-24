{
  description = "Rust toolchain";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      overlays = [
        (import rust-overlay)
        (self: super: {
          rustToolchain = super.rust-bin.stable.latest.default;
        })
      ];

      allSystems = [
        "aarch64-darwin" # 64-bit macOS ARM
        "aarch64-linux"  # 64-bit Linux ARM
        "x86_64-linux"   # 64-bit Linux Intel/AMD
        "x86_64-darwin"  # 64-bit macOS Intel
      ];

      forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
        pkgs = import nixpkgs { inherit overlays system; };
      });
    in
    {
      devShells = forAllSystems ({ pkgs }: {
        default = pkgs.mkShell {
          buildInputs = with pkgs; ([
            # packages provided by the Rust overlay include:
            #   cargo, Clippy, cargo-fmt, rustdoc, rustfmt
            rustToolchain
          ] ++ lib.optionals stdenv.isDarwin [
            darwin.apple_sdk.frameworks.Security
            libiconv # solves the "missing -liconv" issue when running cargo
          ]);

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

          packages = with pkgs; [
            just
          ];

          shellHook = ''
            echo "
              Rust Toolchain   = ${pkgs.rustToolchain}/bin
              Rust Source Path = $RUST_SRC_PATH
            "

            # health checks for Nix flake inputs
            nix run "github:DeterminateSystems/flake-checker"

#            rustup default stable
#            rustup component add rust-src
            git submodule update --init
          '';
        };
      });
    };
}

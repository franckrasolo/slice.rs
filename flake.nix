{
  description = "Rust toolchain";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
      follows = "rust-overlay/nixpkgs";
    };
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
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

          packages = (with pkgs; [
            just
            # packages provided by the Rust overlay include:
            #   cargo, Clippy, cargo-fmt, rustdoc, rustfmt
            rustToolchain
          ]) ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [
            darwin.apple_sdk.frameworks.Security
            libiconv
          ]);

#          shellHook = ''
#            rustup default stable
#            rustup component add rust-src
#          '';
#          buildInputs = with pkgs; [
#            # needed for cargo - solves the "missing -liconv" issue
#            libiconv
#          ];
        };
      });
    };
}

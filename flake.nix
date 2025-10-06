{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };
  outputs =
    { nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          overlays = [ (import inputs.rust-overlay) ];
          inherit system;
        };
        buildInputs = with pkgs; [
          openssl
          udev
          alsa-lib-with-plugins
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          libxkbcommon
          wayland
        ];
        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        stableRust = (
          pkgs.rust-bin.stable."1.90.0".default.override {
            extensions = [
              "rust-analyzer"
              "rust-src"
            ];
          }
        );
        nightlyRust = pkgs.rust-bin.selectLatestNightlyWith (
          toolchain:
          toolchain.default.override {
            extensions = [
              "rust-analyzer"
              "rust-src"
            ];
          }
        );
      in
      {
        devShells = {
          default = pkgs.mkShell {
            inherit nativeBuildInputs LD_LIBRARY_PATH;
            buildInputs = buildInputs ++ [ stableRust ];
          };
          docrs = pkgs.mkShell {
            inherit nativeBuildInputs LD_LIBRARY_PATH;
            buildInputs = buildInputs ++ [ nightlyRust ];
          };
        };
      }
    );
}

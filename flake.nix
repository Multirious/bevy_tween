{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    inputs@{ nixpkgs, ... }:
    let
      inherit (nixpkgs) lib;
      eachSystem = lib.genAttrs [ "x86_64-linux" ];
      pkgsFor = eachSystem (
        system:
        import nixpkgs {
          inherit system;
          overlays = [
            (import inputs.rust-overlay)
          ];
        }
      );
      forAllSystem = f: lib.mapAttrs f pkgsFor;
    in
    {
      devShells = forAllSystem (
        system: pkgs:
        let
          buildInputs = with pkgs; [
            openssl
            udev
            alsa-lib-with-plugins
            vulkan-loader
            libX11
            libXcursor
            libXi
            libXrandr
            libxkbcommon
            wayland
          ];
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          stableRust = pkgs.rust-bin.stable.latest.default.override {
            extensions = [
              "rust-analyzer"
              "rust-src"
            ];
          };
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
          default = pkgs.mkShell {
            inherit nativeBuildInputs LD_LIBRARY_PATH;
            buildInputs = buildInputs ++ [ stableRust ];
          };
          docrs = pkgs.mkShell {
            inherit nativeBuildInputs LD_LIBRARY_PATH;
            buildInputs = buildInputs ++ [ nightlyRust ];
          };
        }
      );
    };
}

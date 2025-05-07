{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay= {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, ... } @ inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          overlays = [ (import inputs.rust-overlay) ];
          inherit system;
        };
        buildInputs = with pkgs; [
          openssl
          udev alsa-lib-with-plugins vulkan-loader
          xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
          libxkbcommon wayland
          (rust-bin.stable."1.85.1".default.override {
            extensions = ["rust-analyzer" "rust-src"];
          })
        ];
        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      in
      {
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs LD_LIBRARY_PATH;
        };
      }
    );
}

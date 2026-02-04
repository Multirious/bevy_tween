{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    inputs:
    let
      forAllSystems =
        function:
        inputs.nixpkgs.lib.genAttrs
          [
            "x86_64-linux"
          ]
          (
            system:
            function (
              import inputs.nixpkgs {
                inherit system;
                overlays = [
                  (import inputs.rust-overlay)
                ];
              }
            )
          );
    in
    {
      devShells = forAllSystems (
        pkgs:
        let
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

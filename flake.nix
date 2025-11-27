{
  description = "TUI color-picker (Rust)";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        tui-color-picker = pkgs.rustPlatform.buildRustPackage {
          pname = "tui-color-picker";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          postInstall = ''
            strip $out/bin/tui-color-picker
          '';
        };
      in {
        packages.default = tui-color-picker;
        apps.default = flake-utils.lib.mkApp { drv = tui-color-picker; };
      });
}

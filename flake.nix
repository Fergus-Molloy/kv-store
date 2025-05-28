{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      naersk,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        naersk' = pkgs.callPackage naersk { };

      in
      {
        packages = rec {
          message-hub = naersk'.buildPackage {
            src = ./.;
          };
          test = naersk'.buildPackage {
            src = ./.;
            mode = "test";
          };
          check = naersk'.buildPackage {
            src = ./.;
            mode = "clippy";
            cargoClippyOptions =
              default:
              default
              ++ [
                "-Dclippy::correctness"
                "-Wclippy::complexity"
                "-Wclippy::perf"
                "-Wclippy::pedantic"
              ];
          };
          default = message-hub;
        };
        devShells = {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              pkg-config
              rust-bin.stable.latest.default
              rust-analyzer
              cargo-nextest
              clang
              llvmPackages.bintools
              watchexec
              just
            ];
          };

        };
      }
    );
}

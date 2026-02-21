{
  description = "dfhack-remote - Rust client for the DFHack remote API";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachSystem nixpkgs.lib.systems.flakeExposed (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };

        rustToolchain = (
          pkgs.rust-bin.selectLatestNightlyWith (t: t.default)
        ).override {
          extensions = ["rust-src" "rust-analyzer"];
        };

        # Build inputs needed when regenerating protos
        # (reqwest uses native-tls which needs OpenSSL on Linux)
        regenBuildInputs = with pkgs;
          lib.optionals stdenv.isLinux [
            openssl
            pkg-config
          ];

        nativeBuildInputs = with pkgs;
          lib.optionals stdenv.isLinux [
            mold
          ];

        rustLinkerFlags =
          if pkgs.stdenv.isLinux
          then ["-fuse-ld=mold" "-Wl,--compress-debug-sections=zstd"]
          else if pkgs.stdenv.isDarwin
          then ["--ld-path=$(unset DEVELOPER_DIR; /usr/bin/xcrun --find ld)" "-ld_new"]
          else [];

        rustLinkFlagsString =
          pkgs.lib.concatStringsSep " "
          (pkgs.lib.concatMap (x: ["-C" "link-arg=${x}"]) rustLinkerFlags);
      in {
        formatter = pkgs.alejandra;

        apps = {
          lint = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "lint" ''
              exec ${rustToolchain}/bin/cargo clippy --workspace "$@"
            '';
          };

          test = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "test" ''
              exec ${rustToolchain}/bin/cargo test "$@"
            '';
          };

          regen = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "regen" ''
              export DFHACK_DOWNLOAD=1
              export DFHACK_REGEN=1
              exec ${rustToolchain}/bin/cargo build "$@"
            '';
          };
        };

        devShells.default = pkgs.mkShell {
          name = "dfhack-remote";

          packages =
            [
              rustToolchain
              pkgs.cargo-release
            ]
            ++ nativeBuildInputs
            ++ regenBuildInputs;

          env = {
            RUST_BACKTRACE = 1;
            CARGO_INCREMENTAL = "0";
          };

          shellHook = ''
            export RUSTFLAGS="-Zthreads=0 ${rustLinkFlagsString}"
          '';
        };
      }
    );
}

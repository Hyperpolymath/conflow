{
  description = "conflow - Configuration Flow Orchestrator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Common build inputs
        buildInputs = with pkgs; [
          openssl
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.darwin.apple_sdk.frameworks.Security
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        # Build conflow
        conflow = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;
          inherit buildInputs nativeBuildInputs;
        };
      in
      {
        packages = {
          default = conflow;
          conflow = conflow;
        };

        apps = {
          default = flake-utils.lib.mkApp {
            drv = conflow;
          };
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ conflow ];

          packages = with pkgs; [
            # Rust toolchain
            rustToolchain
            rust-analyzer

            # Build tools
            just
            cargo-watch
            cargo-audit
            cargo-outdated

            # Config tools (for testing)
            cue
            nickel

            # Development utilities
            git
            jq
            yq
          ];

          shellHook = ''
            echo "conflow development shell"
            echo "  Rust: $(rustc --version)"
            echo "  CUE:  $(cue version 2>/dev/null | head -1 || echo 'not installed')"
            echo "  Nickel: $(nickel --version 2>/dev/null || echo 'not installed')"
            echo ""
            echo "Commands:"
            echo "  just        - Show available recipes"
            echo "  just build  - Build the project"
            echo "  just test   - Run tests"
            echo "  just check  - Run all checks"
            echo ""
          '';

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };

        # Checks for CI
        checks = {
          inherit conflow;

          fmt = craneLib.cargoFmt {
            src = craneLib.cleanCargoSource ./.;
          };

          clippy = craneLib.cargoClippy {
            src = craneLib.cleanCargoSource ./.;
            inherit buildInputs nativeBuildInputs;
            cargoClippyExtraArgs = "--all-targets -- -D warnings";
          };

          test = craneLib.cargoTest {
            src = craneLib.cleanCargoSource ./.;
            inherit buildInputs nativeBuildInputs;
          };
        };
      });
}

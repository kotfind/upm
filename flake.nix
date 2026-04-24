{
  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    duct-py-src = {
      url = "github:oconnor663/duct.py";
      flake = false;
    };
    cbor-diag-cli-src = {
      url = "github:Nullus157/cbor-diag-rs";
      flake = false;
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    fenix,
    duct-py-src,
    cbor-diag-cli-src,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};

      inherit (pkgs) mkShell makeRustPlatform;

      rustToolchain = with fenix.packages.${system};
        combine (
          (with stable; [
            rustc
            cargo
            rustfmt
            clippy
            rust-src
            rust-analyzer
          ])
          ++ (map (t: targets.${t}.stable.rust-std) [
            "thumbv8m.main-none-eabihf"
            "riscv32imac-unknown-none-elf"
            "thumbv6m-none-eabi"
            "x86_64-unknown-linux-gnu"
          ])
        );

      ductPy = pypkgs:
        with pypkgs;
          buildPythonPackage {
            pname = "duct";
            version = "1.0.1";

            src = duct-py-src;
            pyproject = true;

            buildInputs = with pypkgs; [
              hatchling
            ];
          };

      rustPlatform = makeRustPlatform {
        cargo = rustToolchain;
        rustc = rustToolchain;
      };

      cborDiagCli = rustPlatform.buildRustPackage (finalAttrs: {
        pname = "cbor-diag-cli";
        version = "0.1.8";

        src = cbor-diag-cli-src;

        cargoHash = "sha256-Paf53r3i8rXvlEDk8m10NsdsW5axJlHhTMEms6SUPLc=";
      });

      python = pkgs.python3.withPackages (
        pypkgs:
          [(ductPy pypkgs)]
          ++ (with pypkgs; [
            click
            pyserial
          ])
      );

      shell = mkShell {
        name = "upm-shell";

        buildInputs =
          [
            rustToolchain
            python
            cborDiagCli
          ]
          ++ (with pkgs; [
            cargo-machete
            cargo-autoinherit
            cargo-expand

            flip-link
            picotool
            picocom
          ]);
      };
    in {
      devShells.default = shell;
    });
}

{
  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    duct-py-src = {
      url = "https://files.pythonhosted.org/packages/source/d/duct/duct-1.0.1.tar.gz";
      flake = false;
    };
    cbor-diag-cli-src = {
      url = "http://crates.io/api/v1/crates/cbor-diag-cli/0.1.8/download";
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

      inherit (pkgs) mkShell;

      rustToolchain = with fenix.packages.${system};
        combine (
          (with stable; [
            rustc
            cargo
            rustfmt
            clippy
            rust-src
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

      cborDiagCli = pkgs.rustPlatform.buildRustPackage (finalAttrs: {
        pname = "cbor-diag-cli";
        version = "0.1.8";

        src = cbor-diag-cli-src;
        unpackCmd = "tar xf $src";

        cargoHash = "sha256-jHy7D7xlKoQLKvLxqL8pzjvUx1fUvLxz0tx0QLStJUY=";
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
        name = "usb-password-manager-shell";

        buildInputs =
          [
            rustToolchain
            python
            cborDiagCli
          ]
          ++ (with pkgs; [
            rust-analyzer

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

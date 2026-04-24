{
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    duct-py-src = {
      url = "github:oconnor663/duct.py";
      flake = false;
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    duct-py-src,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;

        # for corefonts
        config.allowUnfree = true;
      };
      lib = pkgs.lib;

      inherit (pkgs) mkShell;
      inherit (lib) concatMapStringsSep;

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

      python = pkgs.python3.withPackages (pypkgs:
        [(ductPy pypkgs)]
        ++ (with pypkgs; [
          click
        ]));

      fonts = with pkgs; [
        corefonts
        dejavu_fonts
      ];

      shell = mkShell {
        name = "upm-doc-shell";

        buildInputs =
          [python]
          ++ fonts
          ++ (with pkgs; [
            typst
          ]);

        TYPST_FONT_PATHS =
          concatMapStringsSep
          ":"
          (font_pkg: "${font_pkg}/share/fonts/truetype/")
          fonts;
      };
    in {
      devShell = shell;
    });
}

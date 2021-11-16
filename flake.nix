{
  inputs = {
    nixpkgs.url = github:nixos/nixpkgs;
    flake-utils.url = github:numtide/flake-utils;
    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      rs = pkgs.rust.packages.stable;
    in
    {
      devShell = pkgs.mkShell {
        nativeBuildInputs = [
          rs.cargo
          rs.rustc
          rs.rustfmt
        ];
        buildInputs = [
          pkgs.libiconv
        ];

        RUST_SRC_PATH = "${rs.rustPlatform.rustLibSrc}";
      };
    });
}

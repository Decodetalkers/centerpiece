{
  description = "Your trusty omnibox search.";

  inputs = { nixpkgs.url = "github:NixOS/nixpkgs"; };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

      devInputs = with pkgs; [ rustc rustfmt cargo ];

      nativeBuildInputs = with pkgs;
        [
          # cmake pkgconf
          makeWrapper
        ];

      buildInputs = with pkgs; [ ];
    in {
      devShells.${system}.default = pkgs.mkShell {
        inherit nativeBuildInputs buildInputs;
        packages = devInputs;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.wayland
          pkgs.libxkbcommon
          pkgs.vulkan-loader
          pkgs.libGL
        ];
      };

      packages.${system}.default = pkgs.rustPlatform.buildRustPackage rec {
        pname = "centerpiece";
        version = "0.6.0";

        inherit nativeBuildInputs buildInputs;

        postInstall = ''
          wrapProgram "$out/bin/${pname}" \
            --prefix LD_LIBRARY_PATH : ${
              pkgs.lib.makeLibraryPath [
                pkgs.wayland
                pkgs.libxkbcommon
                pkgs.vulkan-loader
                pkgs.libGL
              ]
            }
        '';

        src = ./.;

        cargoLock.lockFile = ./Cargo.lock;

        meta = with pkgs.lib; {
          description = "Your trusty omnibox search.";
          homepage = "https://github.com/friedow/centerpiece";
          platforms = platforms.linux;
          license = licenses.mit;
          maintainers = [ "friedow" ];
        };
      };
    };
}

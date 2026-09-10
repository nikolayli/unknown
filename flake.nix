{
  description = "unknown";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        runtimeLibs = with pkgs; [
          wayland
          libxkbcommon

          libX11
          libXcursor
          libXrandr
          libXi
          libXxf86vm
          libGL
          vulkan-loader
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            cargo
            rustc
          ];

          buildInputs = runtimeLibs ++ (with pkgs; [ glibcLocales ]);

          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath runtimeLibs}:$LD_LIBRARY_PATH"
            export DISPLAY="''${DISPLAY:-:0}"
          '';
        };
      }
    );
}

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

        # Исправленные актуальные имена пакетов без xorg.
        runtimeLibs = with pkgs; [
          libX11
          libXcursor
          libXrandr
          libXi
          libXxf86vm
          libGL
          vulkan-loader # Добавляем для работы с Vulkan
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
            # Автоматический проброс путей к динамическим библиотекам
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath runtimeLibs}:$LD_LIBRARY_PATH"

            # Если вы сидите на Wayland, но запускаете X11-окно через XWayland
            export DISPLAY="''${DISPLAY:-:0}"
          '';
        };
      }
    );
}

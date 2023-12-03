{
  description = "Nix development environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs, ... }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in {
    devShells.${system}.default = with pkgs; mkShell rec {
      inputsFrom = [
        # Wayland libraries
        wayland

        # X11 libraries
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        libxkbcommon

        # Vulkan libraries
        shaderc
        vulkan-loader
        vulkan-validation-layers
      ];
      inputPath = lib.makeLibraryPath inputsFrom;
      shellHook = ''
        export LD_LIBRARY_PATH=${inputPath};
        export SHADERC_LIB_DIR=${inputPath};
        export VK_LAYER_PATH="${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d";
      '';
    };
  };
}

{
  description = "Flexy nix flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {nixpkgs, ...}:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
    buildInputs = with pkgs; [
      fontconfig.dev
      fontconfig
      libxkbcommon.dev
      wayland
      libxkbcommon
      xorg.libxcb
      xorg.libX11
      xorg.libXcursor
      xorg.libXrandr
      xorg.libXi
      lua5_4
      pkgs.libGL
    ];
  in
  {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs;
      [libxkbcommon] ++ buildInputs;
      LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
      PKG_CONFIG_PATH = "${pkgs.fontconfig.dev}/lib/pkgconfig:${pkgs.lua5_4}/lib/pkgconfig:${pkgs.lib.makeLibraryPath buildInputs}";
    };
  };
}

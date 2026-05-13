{
  description = "Motor DSP y FFI para PipeWire - Entorno Aislado";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        # Dependencias de compilación y herramientas de C
        nativeBuildInputs = with pkgs; [
          pkg-config
          clang
        ];

        # Librerías y el Toolchain de Rust empaquetado por Nix
        buildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          pipewire
        ];

        # Variables de entorno críticas para que FFI y el Linker funcionen en NixOS
        LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
        
        # Le dice al ejecutable dónde encontrar PipeWire en tiempo de ejecución
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
          pipewire
        ]);
      };
    };
}
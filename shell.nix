{ pkgs ? import <nixpkgs> {} }:

with pkgs;
mkShell {
  buildInputs = [
    emscripten
    nodejs
    rustup
    SDL2
  ];
}

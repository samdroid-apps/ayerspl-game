#!/usr/bin/env fish

set -x LD_LIBRARY_PATH (nix eval --raw -f library-path.nix name):$LD_LIBRARY_PATH
set -x PKG_CONFIG_PATH (nix eval --raw nixpkgs.alsaLib.dev)/lib/pkgconfig:$PKG_CONFIG_PATH
set -x FISH_P1 "(game) "
nix run nixpkgs.rustup \
  nixpkgs.gcc \
  nixpkgs.binutils-unwrapped \
  nixpkgs.pkgconfig \
  nixpkgs.alsaLib \
  nixpkgs.alsaLib.dev \
  --command fish

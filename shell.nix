{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
      sqlx-cli
      rustup
      bacon
      dig
      gcc
      pkg-config
      openssl
      postgresql
    ];
  }
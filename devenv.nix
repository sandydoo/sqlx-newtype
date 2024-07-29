{ pkgs, lib, config, inputs, ... }:

{
  languages.rust.enable = true;

  env.DATABASE_URL = "sqlite:sqlx-example.db";

  packages = [
    pkgs.sqlite
    pkgs.sqlx-cli
  ] ++ lib.optionals pkgs.stdenv.isDarwin [
    pkgs.darwin.apple_sdk.frameworks.Security
  ];
}

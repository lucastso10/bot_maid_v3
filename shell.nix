{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  name = "bot maid v3";
  # Additional tooling
  buildInputs = with pkgs; [
    rustfmt       # Formatter
    cargo
    rustc
  ];

  # Create a file named .token with the token to your discord bot!
  shellHook = "export DISCORD_TOKEN=" + import ./token.nix;
}

{
  description = "Nix flake for tauri development";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
  in {
    devShells."x86_64-linux".default = pkgs.mkShell {
      buildInputs = with pkgs; [
        pkg-config
        gobject-introspection
        cargo 
        rustc 
        rustfmt 
        clippy 
        rust-analyzer 
        nodejs
        nodePackages.pnpm

        at-spi2-atk
        atkmm
        cairo
        gdk-pixbuf
        glib
        gtk3
        gsettings-desktop-schemas
        harfbuzz
        librsvg
        libsoup_3
        webkitgtk_4_1
        openssl
      ];

      native_build_inputs = [ pkgs.pkg-config ];
      env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";


      shellHook = ''
        export XDG_DATA_DIRS=\
      ${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:\
      ${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:\
      $XDG_DATA_DIRS
      '';
    };
  };
}

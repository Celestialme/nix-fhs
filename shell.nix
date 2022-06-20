{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    
    nativeBuildInputs = [ 
      pkgs.gcc
      pkgs.rustc
      pkgs.rustfmt
      pkgs.cargo
      pkgs.cargo-edit
      pkgs.rust-analyzer
     

     ];
     buildInputs = [ 
      pkgs.cargo
      pkgs.dpkg
      pkgs.wget
      pkgs.nix-index
     ];
}
# http://ftp.de.debian.org/debian/pool/main/h/htop/htop_3.2.1-1_amd64.deb
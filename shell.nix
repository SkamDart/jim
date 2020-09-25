with import <nixpkgs> {};

# Uses Mozilla Rust Overlay
let
  crust = (rustChannels.stable.rust.override { extensions = [ "rust-src" ]; });
in
stdenv.mkDerivation {
  name = "jim";
  buildInputs = with pkgs; [ pkgconfig openssl crust rustracer ];
  RUST_SRC_PATH = "${crust}/lib/rustlib/src/rust/src";
}

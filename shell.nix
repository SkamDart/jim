with import <nixpkgs> {};

# Uses Mozilla Rust Overlay
let
  crust = (rustChannels.stable.rust.override { extensions = [ "rust-src" ]; });
in
stdenv.mkDerivation {
  name = "andurilapis-rs";
  buildInputs = with pkgs; [ pkgconfig openssl crust protobuf rustracer ];
  RUST_SRC_PATH = "${crust}/lib/rustlib/src/rust/src";
  PROTOC = "${protobuf}/bin/protoc";
  PROTOC_INCLUDE = "${protobuf}/include";
}

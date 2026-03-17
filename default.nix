{
  rustPlatform,
  pkg-config,
}:
rustPlatform.buildRustPackage {
  name = "chara";
  src = ./.;
  nativeBuildInputs = [ pkg-config ];
  cargoLock.lockFile = ./Cargo.lock;
  # buildInputs = [ if we ever need smt ];
}

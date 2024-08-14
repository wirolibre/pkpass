{ lib

, rustPlatform
, gitignore

, pkg-config
, openssl
}:

let
  inherit (gitignore.lib) gitignoreSource;

  src = gitignoreSource ./.;
  manifest = lib.importTOML "${src}/Cargo.toml";

  pkg-info = manifest.workspace.package;
in
rustPlatform.buildRustPackage {
  pname = "pkpass";
  version = "0.1.0";

  inherit src;

  cargoLock = { lockFile = "${src}/Cargo.lock"; };

  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    openssl
  ];

  meta = {
    inherit (pkg-info) homepage license;
    description = "pkpass file format toolchain";
    maintainers = pkg-info.authors;
    mainProgram = "pkpass";
  };
}

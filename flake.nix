{
  description = "Rust-based full-stack wedding application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        # Rust development environment
        rustEnv = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustup # Rust toolchain manager
            cargo # Rust build system
            wasm-pack # For building the Yew frontend
            trunk # For serving Yew apps
            openssl # OpenSSL with headers
            pkg-config # Required to locate OpenSSL
            sqlite # SQLite for the database
            docker # For building Docker images
            sqlx-cli # For creating sqlx migrations
            cargo-leptos
            cargo-generate
            sass
            diesel-cli
          ];

          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

          shellHook = ''
            echo "Setting up the Rust development environment..."

            # Configure Rust toolchain
            rustup default stable
            rustup target add wasm32-unknown-unknown
            echo "Environment ready. To start:"
            echo "  - 'cd backend && cargo run' for the backend."
            echo "  - 'cd frontend && trunk serve' for the frontend."
          '';
        };
      in
      {
        # Development shell
        devShell = rustEnv;
      }
    );
}

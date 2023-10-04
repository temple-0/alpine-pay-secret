curl -fsSL https://github.com/scrtlabs/SecretNetwork/releases/download/v1.11.0/secretcli-Linux -o secretcli
chmod +x secretcli
sudo mv secretcli /usr/local/bin/secretcli
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script

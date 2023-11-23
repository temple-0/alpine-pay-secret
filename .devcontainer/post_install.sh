apt update && apt upgrade -y && apt install -y dos2unix
curl -fsSL https://github.com/scrtlabs/SecretNetwork/releases/download/v1.11.0/secretcli-Linux -o secretcli
dos2unix secretcli*
chmod +x secretcli
mv secretcli /usr/local/bin/secretcli
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script

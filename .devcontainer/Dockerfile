# FROM docker:dind
# RUN apk update && apk upgrade 
# RUN apk add ca-certificates curl openssl-dev
# RUN curl -fsSL https://github.com/scrtlabs/SecretNetwork/releases/download/v1.11.0/secretcli-Linux -o secretcli
# RUN chmod +x secretcli
# RUN mv secretcli /usr/local/bin/secretcli
# RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
# RUN source ~/.cargo/env
# ENV PATH="/root/.cargo/bin:${PATH}"
# RUN rustup default stable
# RUN rustup target add wasm32-unknown-unknown
# RUN apk add libgcc build-base openssl
# # RUN apk add pkg-config 
# # RUN cargo install cargo-generate --features vendored-openssl
# # RUN cargo install cargo-run-script
FROM rust:bookworm
RUN apt update && apt install lsb-release -y
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
RUN echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null
RUN apt update && apt upgrade -y
RUN apt install docker-ce docker-ce-cli containerd.io -y
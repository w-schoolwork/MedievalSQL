FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld dnsutils \
   && apt-get autoremove -y && apt-get clean -y

RUN cargo install sqlx-cli cargo-watch
RUN chmod -R 777 /usr/local/cargo
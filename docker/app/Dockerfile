FROM ubuntu:22.04

RUN apt-get update && \
    apt-get install -y \
        curl \
        git \
        build-essential \
        pkg-config \
        libpq-dev \
        postgresql-client

ARG UID
ARG GID
RUN groupadd -g $GID vscode
RUN useradd vscode -u $UID -g $GID -m
USER vscode

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain=stable -c rust-src -y
ENV PATH=$PATH:/home/vscode/.cargo/bin
RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /home/vscode/workspace
CMD ["sleep", "infinity"]

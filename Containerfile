FROM registry.access.redhat.com/ubi9/ubi:latest

ARG RUST_VERSION="1.83.0"

RUN dnf install -y gcc openssl openssl-devel cmake gcc-c++ git curl-minimal

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain ${RUST_VERSION}

ENV PATH "$PATH:/root/.cargo/bin"

RUN mkdir /usr/src/project

COPY . /usr/src/project

WORKDIR /usr/src/project

RUN cargo build --release

ENTRYPOINT ["./target/release/otel-actix-example"]

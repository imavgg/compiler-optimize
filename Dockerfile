FROM rust:1.47

RUN cargo install cargo-binutils
RUN rustup component add llvm-tools-preview

WORKDIR /workspace

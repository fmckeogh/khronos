FROM rust AS builder
WORKDIR /workdir

# prepare toolchain
RUN rustup target add x86_64-unknown-linux-musl

# add musl tools
RUN apt-get update && apt-get install musl-tools clang llvm -y

# fetch registry
RUN cargo init --bin .
RUN cargo build

# build dependencies
COPY Cargo.lock .
COPY Cargo.toml .
RUN cargo build --release --target x86_64-unknown-linux-musl

# build app
COPY . .
RUN touch src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /workdir/target/x86_64-unknown-linux-musl/release/khronos .
ENTRYPOINT ["./khronos"]

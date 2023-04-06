FROM rust:alpine as build

RUN apk add --no-cache \
    ca-certificates \
    cmake \
    curl \
    git \
    sqlite-dev \
    musl-dev \
    openssl-dev \
    pkgconfig \
    clang

ENV LANG=C.UTF-8
ENV LC_ALL=C.UTF-8


# Build
WORKDIR /build

# Copy the rust-toolchain.toml file and install the Rust toolchain
COPY rust-toolchain.toml .
RUN cargo --version

# Install the Leptos build helper CLI
RUN cargo install cargo-leptos

COPY . .

# Build the application
RUN cargo leptos build --release

# Clean up the SQLite database and make the server executable
RUN sqlite3 ./target/default.sqlite3 "VACUUM;"
RUN chmod +x ./target/server/release/server

# this distroless base image is just ~20MB
FROM gcr.io/distroless/cc-debian11

COPY --from=build /build/target/default.sqlite3 ./target/default.sqlite3
COPY --from=build /build/target/site ./target/site
COPY --from=build /build/target/server/release/server ./server

# total docker image size is ~335B
ENTRYPOINT ["./server"]
CMD []

FROM ghcr.io/rust-lang/rust:nightly-bullseye-slim as build

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    cmake \
    curl \
    git \
    sqlite3 \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

ENV LANG=C.UTF-8
ENV LANGUAGE=C.UTF-8
ENV LC_ALL=C.UTF-8

# install leptos build helper cli
RUN cargo install --locked cargo-leptos

# Build
WORKDIR /build

COPY . .

RUN cargo leptos build --release

RUN sqlite3 ./target/default.sqlite3 "VACUUM;"

# this distroless base image is just ~22MB
FROM gcr.io/distroless/cc-debian11

COPY --from=build /build/target/default.sqlite3 ./target/default.sqlite3
COPY --from=build /build/target/site ./target/site
COPY --from=build /build/target/server/release/server ./server

# total docker image size is ~33MB
ENTRYPOINT ["./server"]
CMD []

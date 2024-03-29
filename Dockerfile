FROM ghcr.io/rust-lang/rust:nightly-bullseye-slim as build

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    cmake \
    curl \
    git \
    sqlite3 \
    npm \
    libssl-dev \
    pkg-config \
    clang \
    gcc \
    make \
    && rm -rf /var/lib/apt/lists/*

ENV LANG=C.UTF-8
ENV LANGUAGE=C.UTF-8
ENV LC_ALL=C.UTF-8

# build & install mold linker
RUN git clone https://github.com/rui314/mold.git -b v1.11.0 --depth 1 && \
    mkdir mold/build && \
    cd mold/build && \
    ../install-build-deps.sh && \
    cmake -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ .. && \
    cmake --build . -j $(nproc) && \
    cmake --install . && \
    cd ../../ && \
    rm -rf mold


# install leptos build helper cli
RUN cargo install --git https://github.com/akesson/cargo-leptos cargo-leptos --force

ADD ./package.json ./package.json
ADD ./package-lock.json ./package-lock.json

RUN npm install

# Build
WORKDIR /build

# install rust toolchain
COPY rust-toolchain.toml .
RUN cargo --version

COPY . .
COPY .cargo/docker-config.toml .cargo/config.toml

RUN cargo leptos build --release

RUN mkdir -p ./out/target \
    && cp -r ./target/site                                              ./out/target/site \
    && cp -r ./target/server/x86_64-unknown-linux-gnu/release/server    ./out/server

RUN chmod +x ./out/server
RUN sqlite3 ./out/default.sqlite3 "VACUUM;"

FROM gcr.io/distroless/static-debian11

WORKDIR /app
COPY --from=build /build/out ./

ENTRYPOINT ["./server"]
CMD []

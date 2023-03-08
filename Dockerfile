FROM ubuntu:latest as build

ENV LANG=C.UTF-8
ENV LANGUAGE=C.UTF-8
ENV LC_ALL=C.UTF-8

# rust deps
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    ca-certificates curl file \
    build-essential \
    autoconf automake autotools-dev libtool xutils-dev && \
    rm -rf /var/lib/apt/lists/*

ENV SSL_VERSION=1.0.2u

RUN curl https://www.openssl.org/source/openssl-$SSL_VERSION.tar.gz -O && \
    tar -xzf openssl-$SSL_VERSION.tar.gz && \
    cd openssl-$SSL_VERSION && ./config && make depend && make install && \
    cd .. && rm -rf openssl-$SSL_VERSION*

ENV OPENSSL_LIB_DIR=/usr/local/ssl/lib \
    OPENSSL_INCLUDE_DIR=/usr/local/ssl/include \
    OPENSSL_STATIC=1

# install toolchain
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain nightly-2023-01-06 -y

ENV PATH=/root/.cargo/bin:$PATH

# build deps
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    sqlite3 apt-utils gcc g++ make npm \
    libasound2-dev libudev-dev pkg-config mold clang && \
    rm -rf /var/lib/apt/lists/*

# cargo make
RUN cargo install --locked cargo-leptos


# Build
WORKDIR /build

COPY . .

RUN cargo leptos build --release

RUN sqlite3 ./target/default.sqlite3 "VACUUM;"

RUN ls

# minimal RUN
#FROM alpine:3.16.2
FROM ubuntu:latest

WORKDIR /pkg

RUN mkdir /pkg/target

COPY --from=build /build/target/site /pkg/target/site
COPY --from=build /build/target/default.sqlite3 /pkg/target/default.sqlite3
COPY --from=build /build/target/server/release/server /pkg/server

RUN chmod +x ./server

ENTRYPOINT ["./server"]
CMD []

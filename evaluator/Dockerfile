# ====== Build image ======
FROM rust:1.87-bookworm as builder

WORKDIR /rust/app/
COPY src/ src/
COPY Cargo.lock .
COPY Cargo.toml .
RUN cargo build --release

# ====== nsjail build ======
FROM debian:bookworm-slim as nsjail-builder

WORKDIR /nsjail/

RUN apt-get update && apt-get install -y \
    autoconf \
    bison \
    flex \
    gcc \
    g++ \
    git \
    libprotobuf-dev \
    libnl-route-3-dev \
    libtool \
    make \
    pkg-config \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

RUN git clone https://github.com/google/nsjail.git /nsjail/ && \
    git checkout 3.4 && \
    make

# ====== Runtime image ======
FROM debian:bookworm-slim

WORKDIR /opt/evaluator

ENV PATH="${PATH}:/opt/evaluator/bin"

RUN apt-get update && apt-get install -y \
    libprotobuf32 \
    libnl-route-3-200 \
    curl

# Add compilers that we are too lazy to build
RUN apt-get install -y \
    gcc \
    g++ \
    clang \
    rustc \
    ghc \
    build-essential \
    python3 \
    nodejs \
    racket

COPY --from=builder /rust/app/target/release/evaluator bin/evaluator
COPY --from=nsjail-builder /nsjail/nsjail bin/nsjail
COPY config/ config/
COPY ./scripts/entrypoint.sh scripts/entrypoint.sh

# compilers and interpreters
COPY --from=ghcr.io/gregofi/compilers-cr3000:latest /opt/evaluator/compilers /opt/evaluator/compilers

EXPOSE 7800

CMD ["/opt/evaluator/scripts/entrypoint.sh"]

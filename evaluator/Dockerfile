# ====== Build image ======
FROM rust:1.74-bookworm as builder

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

# ====== GCC build image ======
FROM debian:bookworm-slim as gcc-builder

WORKDIR /opt/evaluator/

COPY ./scripts/setup_cpp.sh ./scripts/setup_cpp.sh

RUN ./scripts/setup_cpp.sh

# ====== Lua build image ======
FROM debian:bookworm-slim as lua-builder

WORKDIR /opt/evaluator/

RUN apt-get update && apt-get install -y \
    wget \
    make \
    gcc \
    libreadline-dev
    
COPY ./scripts ./scripts

RUN ./scripts/setup.sh

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
    racket

COPY --from=builder /rust/app/target/release/evaluator bin/evaluator
COPY --from=nsjail-builder /nsjail/nsjail bin/nsjail

# compilers and interpreters
COPY --from=lua-builder /opt/evaluator/compilers /opt/evaluator/compilers


COPY config/ config/
COPY ./scripts/entrypoint.sh scripts/entrypoint.sh

EXPOSE 7800

CMD ["/opt/evaluator/scripts/entrypoint.sh"]

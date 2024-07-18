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

# ====== Lua build image ======
FROM gcc:13.2-bookworm as lua-builder

WORKDIR /opt/evaluator/
    
COPY ./scripts/setup_lua.sh ./scripts/setup_lua.sh

RUN ./scripts/setup_lua.sh

# ====== Racket build image ======
FROM gcc:13.2-bookworm as racket-builder

COPY ./scripts/setup_racket.sh ./scripts/setup_racket.sh

RUN ./scripts/setup_racket.sh

# ====== Mjolnir build image ======
FROM rust:1.79-bookworm as mjolnir-builder

COPY ./scripts/setup_mjolnir.sh ./scripts/setup_mjolnir.sh

RUN ./scripts/setup_mjolnir.sh

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
COPY config/ config/
COPY ./scripts/entrypoint.sh scripts/entrypoint.sh

# compilers and interpreters
COPY --from=lua-builder /opt/evaluator/compilers/lua /opt/evaluator/compilers/lua
COPY --from=racket-builder /opt/evaluator/compilers/racket /opt/evaluator/compilers/racket
COPY --from=mjolnir-builder /opt/evaluator/compilers/mjolnir /opt/evaluator/compilers/mjolnir

EXPOSE 7800

CMD ["/opt/evaluator/scripts/entrypoint.sh"]

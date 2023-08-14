# ====== Build image ======
FROM rust:alpine

RUN apk --no-cache add ca-certificates musl-dev
WORKDIR /rust/app/
COPY src/ src/
COPY Cargo.lock .
COPY Cargo.toml .
RUN cargo build --release


# ====== Runtime image ======
FROM docker:latest

WORKDIR /www/app/

COPY --from=0 /rust/app/target/release/evaluator ./

RUN apk --no-cache add \
    ca-certificates \
    musl-dev \
    bash \
    openrc \
    curl # for healthcheck TODO: Maybe a better utility to do this

COPY images/ images/
COPY ./scripts/entrypoint.sh ./

EXPOSE 7800

# Use CMD here so that the docker:cli entrypoint
# is executed.
CMD ["./entrypoint.sh"]

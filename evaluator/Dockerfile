# ====== Build image ======
FROM rust:alpine

RUN apk --no-cache add ca-certificates musl-dev
WORKDIR /rust/app/
COPY . .
RUN cargo build --release


# ====== Runtime image ======
FROM alpine:latest

RUN apk --no-cache add ca-certificates musl-dev bash python3

WORKDIR /app/

COPY --from=0 /rust/app/ ./

EXPOSE 7800

CMD ["./target/release/evaluator"]

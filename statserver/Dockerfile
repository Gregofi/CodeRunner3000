FROM golang:1.21.4-alpine3.18 AS builder

WORKDIR /app
COPY go.mod go.sum .
COPY *.go .

RUN go build


FROM alpine:3.18

WORKDIR /app
COPY --from=builder /app/statserver .

ENTRYPOINT [ "./statserver" ]

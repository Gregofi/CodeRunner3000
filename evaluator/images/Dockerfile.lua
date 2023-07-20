FROM alpine

WORKDIR eval_env

RUN apk --no-cache add lua5.1

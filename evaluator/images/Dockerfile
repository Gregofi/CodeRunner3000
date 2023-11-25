FROM alpine:3.18

ENV USER_UID=666
ENV USER_NAME=evaluator_nobody

RUN apk --no-cache add sudo

RUN mkdir -p /evaluator && chmod -R 600 /evaluator

COPY evaluate.sh /evaluator/evaluate.sh

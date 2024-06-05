FROM valkey/valkey:bookworm

ENV PORT=6379
ENV CONFIG_NAME=""

COPY ./configs /etc/configs
COPY ./entrypoint.sh /bin/entrypoint.sh

CMD [ "/bin/entrypoint.sh" ]

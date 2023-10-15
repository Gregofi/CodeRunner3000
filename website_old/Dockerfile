# ====== Frontend build ======
FROM node:20-bookworm

WORKDIR /www/
COPY src/editor-frontend/ editor-frontend/

RUN cd editor-frontend && npm ci && npm run build


# ====== Server runtime image ======
FROM debian:bookworm

ENV WEBSITE_PORT=3113

RUN apt-get update && \
    apt-get install -y \
        python3 \
        python3-flask \
        python3-requests \
        nginx \
        gunicorn \
        gettext-base

COPY conf/nginx.conf.template /etc/nginx/nginx.conf.template

WORKDIR /www

COPY /src .
# The multistage build does not preserve workdir
COPY --from=0 /www/static/ static/
COPY /scripts/entrypoint.sh scripts/

EXPOSE ${WEBSITE_PORT}

ENTRYPOINT ["/www/scripts/entrypoint.sh"]

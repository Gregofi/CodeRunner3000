# ====== Frontend build ======
FROM node:bullseye

WORKDIR /www/
COPY src/editor-frontend/ editor-frontend/

RUN cd editor-frontend && npm ci && npm run build


# ====== Server runtime image ======
FROM debian:bullseye

RUN apt-get update && \
    apt-get install -y \
        python3 \
        python3-pip \
        nginx && \
    pip install \
        flask \
        requests \
        Flask-SQLAlchemy \
        psycopg2-binary \
        pyjwt \
        wtforms

COPY conf/nginx.conf /etc/nginx/nginx.conf

WORKDIR /www

COPY /src .
# The multistage build does not preserve workdir
COPY --from=0 /www/static/ static/
COPY /scripts/entrypoint.sh scripts/

EXPOSE 80

ENTRYPOINT ["/www/scripts/entrypoint.sh"]

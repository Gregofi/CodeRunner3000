# ====== Frontend build ======
FROM debian:bookworm

RUN apt-get update -y && \
    apt-get install -y npm

COPY src/frontend/ /www/frontend/
WORKDIR /www/

RUN cd frontend && npm install && npm run build


# ====== Server runtime image ======
FROM debian:bullseye

RUN apt-get update -y && \
    apt-get install -y \
        python3 \
        python3-pip \
        nginx && \
    pip install flask requests

COPY conf/nginx.conf /etc/nginx/nginx.conf

WORKDIR /www

COPY /src .
# The multistage build does not preserve workdir
COPY --from=0 /www/static/ static/
COPY /scripts/entrypoint.sh scripts/

EXPOSE 80

CMD ["/www/scripts/entrypoint.sh"]

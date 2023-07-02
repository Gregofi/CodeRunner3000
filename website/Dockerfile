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
COPY /scripts/entrypoint.sh scripts/

EXPOSE 80

ENTRYPOINT ["/www/scripts/entrypoint.sh"]

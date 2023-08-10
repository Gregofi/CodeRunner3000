FROM python:3.12.0rc1-alpine3.18

RUN pip install pytest requests

COPY *.py /tests/

WORKDIR /tests

CMD ["pytest"]

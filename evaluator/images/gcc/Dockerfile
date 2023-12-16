FROM debian:bookworm

ARG VERSION=trunk

RUN apt-get update && apt-get install -y \
    gcc \
    make \
    git \
    curl \
    build-essential

WORKDIR /tmp

RUN git clone --verbose --progress https://github.com/gcc-mirror/gcc.git

WORKDIR /tmp/gcc

RUN git checkout "${VERSION}"
RUN ./contrib/download_prerequisites

WORKDIR "/opt/evaluator/compilers/gcc-${VERSION}/build"

ARG CONFIG="--prefix=/opt/evaluator/compilers/gcc-${VERSION} --enable-languages=c,c++ --disable-multilib --disable-bootstrap --target=x86_64-linux-gnu"

RUN /tmp/gcc/configure ${CONFIG}
RUN make -j$(nproc)
RUN make install
RUN rm -rf /tmp/gcc /opt/evaluator/compilers/gcc-${VERSION}/build

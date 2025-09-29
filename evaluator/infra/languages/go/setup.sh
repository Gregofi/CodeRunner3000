#!/bin/bash -e

apt-get update
apt-get install -y wget

COMPILERS_DIR="/opt/evaluator/compilers/go"

setup_go_version() {
    GO_VERSION=$1
    echo "Setting up go version $GO_VERSION"
    GO_FTP="https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz"
    echo "Downloading $GO_FTP"
    wget "${GO_FTP}"
    tar -xzf go${GO_VERSION}.linux-amd64.tar.gz
    mkdir -p "${COMPILERS_DIR}"
    mv go "${COMPILERS_DIR}/${GO_VERSION}"
    GOCACHE="${COMPILERS_DIR}/${GO_VERSION}/.gocache" "${COMPILERS_DIR}/${GO_VERSION}/bin/go" build -v std
}

setup_go_version "1.25.1"
setup_go_version "1.24.7"
setup_go_version "1.23.12"

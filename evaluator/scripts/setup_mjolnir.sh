#!/bin/bash -ev

setup_mjolnir_version() {
    branch=${1:-"master"}
    cd /tmp
    git clone https://github.com/Gregofi/mjolnir.git
    cd mjolnir
    git checkout "${branch}"
    cargo build --release
    mkdir -p "/opt/evaluator/compilers/mjolnir/"
    cp -r /tmp/mjolnir/ "/opt/evaluator/compilers/"
}

setup_mjolnir_version "master"

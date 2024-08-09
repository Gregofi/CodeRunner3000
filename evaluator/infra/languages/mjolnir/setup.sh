#!/bin/bash -ev

setup_mjolnir_version() {
    branch=${1:-"master"}
    git clone https://github.com/Gregofi/mjolnir.git
    cd mjolnir
    git checkout "${branch}"
    cargo build --release --target-dir "/opt/evaluator/compilers/mjolnir/${branch}"
}

setup_mjolnir_version "master"

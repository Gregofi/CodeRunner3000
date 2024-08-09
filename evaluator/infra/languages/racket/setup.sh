#!/bin/bash -ev

setup_racket_version() {
    git clone https://github.com/racket/racket.git
    pushd racket
    git checkout "${1}"
    make base -j
    mkdir -p "/opt/evaluator/compilers/racket/racket-${1}"
    cp -r /tmp/racket/racket/racket "/opt/evaluator/compilers/racket/racket-${1}"
    rm "/opt/evaluator/compilers/racket/racket-${1}/"{src,man,doc} -rf
    popd
}

setup_racket_version "v8.11.1"

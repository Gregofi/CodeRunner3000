#!/bin/bash -ev

setup_racket_version() {
    cd /tmp
    git clone https://github.com/racket/racket.git
    cd racket
    git checkout "${1}"
    make base -j 4
    mkdir -p "/opt/evaluator/compilers/racket/racket-${1}"
    cp -r /tmp/racket/racket "/opt/evaluator/compilers/racket/racket-${1}"
    rm "/opt/evaluator/compilers/racket/racket-${1}/"{src,man,doc} -rf
}

setup_racket_version "v8.11.1"

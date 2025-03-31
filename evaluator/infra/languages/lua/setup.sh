#!/bin/bash -e

# Install various lua packages to /opt/evaluator/runtimes/
# must be run as root

lua_source() {
    LUA_FTP="https://www.lua.org/ftp/lua-${1}.tar.gz"
    echo $LUA_FTP
}

setup_lua_version() {
    LUA_VERSION=$1
    echo "Setting up lua version $LUA_VERSION"
    LUA_FTP=$(lua_source $LUA_VERSION)
    echo "Downloading $LUA_FTP"
    wget $LUA_FTP
    tar -xzf lua-${LUA_VERSION}.tar.gz
    pushd lua-${LUA_VERSION}
    make linux
    mkdir -p /opt/evaluator/compilers/lua/
    cp src/lua "/opt/evaluator/compilers/lua/lua${LUA_VERSION}"
    popd
}

setup_lua_version "5.1.5"
setup_lua_version "5.2.4"
setup_lua_version "5.3.6"
setup_lua_version "5.4.6"

rm -rfd /tmp/evaluator-lua

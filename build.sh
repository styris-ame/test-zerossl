#!/bin/bash

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)

export API_KEY="3bcf584c39b4e48fbd5b82225805f6be"
export RUST_BACKTRACE="1"
export OPENSSL_STATIC="1"
#export -n OPENSSL_STATIC
#export -n OPENSSL_LIB_DIR
#export -n OPENSSL_INCLUDE_DIR
#unset OPENSSL_STATIC
#unset OPENSSL_LIB_DIR
#unset OPENSSL_INCLUDE_DIR
export OPENSSL_LIB_DIR="$SCRIPT_DIR/openssl/lib-linux"
export OPENSSL_INCLUDE_DIR="$SCRIPT_DIR/openssl/include-linux"

cargo test --workspace

@echo off

echo "%~dp0\test"

set "API_KEY=3bcf584c39b4e48fbd5b82225805f6be"
set "RUST_BACKTRACE=1"
REM set "OPENSSL_DIR=%~dp0openssl"
set "OPENSSL_STATIC=1"
set "OPENSSL_LIB_DIR=%~dp0openssl\lib-win"
set "OPENSSL_INCLUDE_DIR=%~dp0openssl\include-win"
cargo test --workspace


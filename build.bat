@echo off

echo "%~dp0\test"
set "PATH=%PATH%;%~dp0openssl\bin"
set "OPENSSL_DIR=%~dp0openssl"
set "API_KEY=3bcf584c39b4e48fbd5b82225805f6be"
set "RUST_BACKTRACE=1"

cargo test --workspace
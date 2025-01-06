@echo off
@echo Define RUSTFLAGS
set RUSTFLAGS=-Ctarget-feature=+crt-static

@echo Build the release
cargo build --release

@echo Copy Release to root directory
copy target\release\*.exe .
@echo off
@echo Define RUSTFLAGS
set RUSTFLAGS=-Ctarget-feature=+crt-static

@echo Build the release
cargo build --release

@echo Copy Release to root directory
copy target\release\*.exe .

@echo Extract version information from Cargo.toml
for /f "tokens=2 delims== " %%i in ('findstr /r /c:"^version" Cargo.toml') do set version=%%i
set version=%version:"=%

@echo Set author information
set author=Gustavo Ota - graviscms@gmail.com

@echo Set license information
set license=For license details, visit https://github.com/GustavoOta/dfe-api

@echo Create version.json
echo { > version.json
echo     "name": "dfe-api", >> version.json
echo     "version": "%version%", >> version.json
echo     "author": "%author%", >> version.json
echo     "license": "%license%" >> version.json
echo } >> version.json

@echo Compress the executable and version.json
powershell -Command "Compress-Archive -Path dfe-api.exe, LICENSE -DestinationPath dfe-api.zip"
@echo off
mkdir build\debug 2>NUL
mkdir build\release 2>NUL

@REM set CARGO_TARGET_DIR=%cd%\build\debug

@REM cargo build         # Для отладочной сборки
@REM # cargo build --release # Для релизной сборки

set CARGO_TARGET_DIR=%cd%\build\ cargo build


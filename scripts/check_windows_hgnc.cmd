@echo off
call C:\tt0106\BuildTools\VC\Auxiliary\Build\vcvars64.bat >nul
if errorlevel 1 exit /b %errorlevel%
set "PATH=C:\Users\Docker\.cargo\bin;C:\tt0106\Python313;%PATH%"
set "RUSTUP_TOOLCHAIN=1.95.0"
set "BIOMCP_BUILD_REVISION=%1"
set "CARGO_TARGET_DIR=C:\h\biomcp-0566-target"
set "CARGO_BUILD_JOBS=2"
cd /d C:\h\biomcp-0566
python scripts\check_hgnc_recovery.py

@echo off
call C:\tt0106\BuildTools\VC\Auxiliary\Build\vcvars64.bat >nul
if errorlevel 1 exit /b %errorlevel%
set "RUSTUP_TOOLCHAIN=1.95.0"
set "CARGO_BUILD_JOBS=2"
set "PATH=C:\Users\Docker\.cargo\bin;C:\Users\Docker\AppData\Roaming\uv\python\cpython-3.13.13-windows-x86_64-none;%PATH%"
cd /d %1
python -c "import json,sys; r=json.load(open('candidate/release.json')); assert r['revision']==sys.argv[1], 'candidate revision differs'" %2
if errorlevel 1 exit /b %errorlevel%
python release\gate.py --artifacts candidate --scratch proof
exit /b %errorlevel%

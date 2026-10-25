@echo off

set name=client
set pwa_path=volume\pwa
set wasm_path=frontend
set project_path=%cd%

cd %pwa_path%

if exist "%name%_bg.wasm" (
    del %name%_bg.wasm
)
if exist "%name%.js" (
    del %name%.js
)
if exist "sw.js" (
    del sw.js
)

cd %project_path%

cargo build --package %wasm_path% --target wasm32-unknown-unknown
if %errorlevel% neq 0 (
    exit /b %errorlevel%
)

move target\wasm32-unknown-unknown\debug\%wasm_path%.wasm target\wasm32-unknown-unknown\debug\%name%.wasm
wasm-bindgen --target web --out-dir wasm-pack\ target\wasm32-unknown-unknown\debug\%name%.wasm
if %errorlevel% neq 0 (
    exit /b %errorlevel%
)

move wasm-pack\%name%_bg.wasm %project_path%\%pwa_path%\
move wasm-pack\%name%.js %project_path%\%pwa_path%\

cd %project_path%\%pwa_path%

if exist "%name%_bg.wasm" (
    echo build %project_path%\%pwa_path%\%name%_bg.wasm successfully
)
if exist "%name%.js" (
    echo build %project_path%\%pwa_path%\%name%.js successfully
)

For /f "tokens=2-4 delims=/ " %%a in ('date /t') do (set date_now=%%c%%a%%b)
For /f "tokens=1-3 delims=/:." %%a in ("%TIME%") do (set time_now=%%a%%b%%c)

echo const VERSION = '%date_now%-%time_now%' > sw.js
type sw_template.js >> sw.js

echo update sw.js version to %date_now%-%time_now%

cd %project_path%

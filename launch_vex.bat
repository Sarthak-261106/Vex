@echo off
setlocal
where py >nul 2>nul
if %errorlevel% equ 0 (
  py -3 "%~dp0run_vex.py"
) else (
  python "%~dp0run_vex.py"
)
endlocal

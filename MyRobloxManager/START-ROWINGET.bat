@echo off
cls
echo ===================================================
echo   🚀 STARTING ROWINGET (COMPLETE SYSTEM)
echo ===================================================
echo.

REM 1. Start Memory Manager (Backend)
echo [1/3] Starting Memory Manager Hub...
cd memory-manager
if not exist "node_modules" (
    echo     [First Run] Installing Backend Dependencies...
    call npm install
)
start "RoWinget Hub" cmd /k "npm run dev"
cd ..

REM 2. Compile Extension (Frontend)
echo [2/3] Checking VS Code Extension...
cd vscode-extension
if not exist "node_modules" (
    echo     [First Run] Installing Extension Dependencies...
    call npm install
)
echo     Ensuring latest build...
call npm run compile
cd ..

REM 3. Launch VS Code
echo [3/3] Opening Visual Studio Code...
echo.
echo ===================================================
echo   SYSTEM IS LIVE!
echo ===================================================
echo.
echo 1. The "RoWinget Hub" window Must stay open.
echo 2. In VS Code, open the "RoWinget" tab (Roblox Icon).
echo 3. In Roblox Studio, open Extensions -> RoWinget.
echo.
code .
exit

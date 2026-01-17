@echo off
REM FormHelper 项目进度管理工具
REM 使用方法:
REM   progress.bat read                    - 读取当前进度
REM   progress.bat start phase1            - 开始阶段1
REM   progress.bat complete phase1         - 完成阶段1
REM   progress.bat task phase1 task_name   - 完成单个任务
REM   progress.bat check phase1            - 检查依赖
REM   progress.bat note "message"          - 添加备注

setlocal enabledelayedexpansion

cd /d "%~dp0"

set PROGRESS_FILE=%~dp0progress.json

if "%1"=="" (
    echo FormHelper 进度管理工具
    echo.
    echo 用法:
    echo   progress.bat read                    - 读取当前进度
    echo   progress.bat start phase1            - 开始阶段1
    echo   progress.bat complete phase1         - 完成阶段1
    echo   progress.bat task phase1 task_name   - 完成单个任务
    echo   progress.bat check phase1            - 检查依赖
    echo   progress.bat note "message"          - 添加备注
    exit /b 0
)

if "%1"=="read" (
    call :read_progress
) else if "%1"=="start" (
    call :start_phase %2
) else if "%1"=="complete" (
    call :complete_phase %2
) else if "%1"=="task" (
    call :complete_task %2 %3
) else if "%1"=="check" (
    call :check_dependencies %2
) else if "%1"=="note" (
    call :add_note %2 %3 %4 %5 %6 %7 %8 %9
) else (
    echo 未知命令: %1
    exit /b 1
)

exit /b 0

:read_progress
echo.
echo ============================================================
echo FormHelper 项目进度
echo ============================================================
echo.

if not exist "%PROGRESS_FILE%" (
    echo 错误: 进度文件不存在
    exit /b 1
)

REM 读取 lastUpdated
for /f "tokens=1,2 delims=:" %%a in ('findstr /c:"lastUpdated" "%PROGRESS_FILE%" 2^>nul') do (
    echo 最后更新: %%b
)
echo.

REM 读取各阶段状态
for %%p in (phase1 phase2 phase3_parallel) do (
    call :read_phase_status %%p
)

echo.
echo ============================================================
exit /b 0

:read_phase_status
set PHASE=%1
set PHASE_NAME=
set STATUS=
set DEP_MET=

REM 获取阶段名称
findstr /c:"\"%PHASE%\"" "%PROGRESS_FILE%" >nul 2>&1
for /f "tokens=1,* delims=:\" %%a in ('findstr /c:"\"%PHASE%\" : {" "%PROGRESS_FILE%" 2^>nul') do (
    for /f "tokens=1,2 delims=:" %%m in ("%%b") do (
        set PHASE_NAME=%%m
    )
)

REM 获取状态
for /f "tokens=1,2 delims=:" %%a in ('findstr /c:"\"status\"" "%PROGRESS_FILE%" ^| findstr /c:"%PHASE%" 2^>nul') do (
    for /f "tokens=1,* delims=," %%s in ("%%b") do (
        set STATUS=%%s
        set STATUS=!STATUS: "=!
        set STATUS=!STATUS: "=!
    )
)

REM 获取依赖状态
call :check_dep_status %PHASE%
if "!DEP_MET!"=="true" (
    echo ^|   ^✓ 依赖满足
) else (
    echo ^|   ^✗ 依赖未满足
)

echo.
exit /b 0

:check_dep_status
set CHECK_PHASE=%1
set DEP_MET=true

REM 检查依赖是否满足
if "%CHECK_PHASE%"=="phase1" (
    set DEP_MET=true
) else if "%CHECK_PHASE%"=="phase2" (
    findstr /c:"\"status\" : \"completed\"" "%PROGRESS_FILE%" | findstr /c:"phase1" >nul 2>&1
    if errorlevel 1 set DEP_MET=false
) else if "%CHECK_PHASE%"=="phase3_parallel" (
    findstr /c:"\"status\" : \"completed\"" "%PROGRESS_FILE%" | findstr /c:"phase2" >nul 2>&1
    if errorlevel 1 set DEP_MET=false
)
exit /b 0

:start_phase
if "%1"=="" (
    echo 用法: progress.bat start ^<phase^>
    exit /b 1
)
echo [开始阶段: %1]
echo 请手动更新 progress.json 中的 status 为 "in_progress"
echo.
exit /b 0

:complete_phase
if "%1"=="" (
    echo 用法: progress.bat complete ^<phase^>
    exit /b 1
)
echo [完成阶段: %1]
echo 请手动更新 progress.json 中的 status 为 "completed"
echo.
exit /b 0

:complete_task
if "%1"=="" (
    echo 用法: progress.bat task ^<phase^> ^<task^>
    exit /b 1
)
if "%2"=="" (
    echo 用法: progress.bat task ^<phase^> ^<task^>
    exit /b 1
)
echo [完成任务: %2 在阶段 %1]
echo 请手动更新 progress.json 中对应任务的状态为 "completed"
echo.
exit /b 0

:check_dependencies
if "%1"=="" (
    echo 用法: progress.bat check ^<phase^>
    exit /b 1
)
echo [检查依赖: %1]
echo.
call :check_dep_status %1
if "!DEP_MET!"=="true" (
    echo ✓ 阶段 %1 可以开始（依赖已满足）
) else (
    echo ✗ 阶段 %1 无法开始（依赖未满足）
)
echo.
exit /b 0

:add_note
echo [添加备注]
echo 请手动在 progress.json 的 notes 数组中添加记录
echo.
exit /b 0

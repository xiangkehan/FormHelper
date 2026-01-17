@echo off
REM 进度管理工具 - FormHelper 项目 (Windows 批处理版本)
REM 使用方法:
REM   progress.bat read                    - 读取当前进度
REM   progress.bat start phase1            - 开始阶段1
REM   progress.bat complete phase1         - 完成阶段1
REM   progress.bat task phase1 task_name   - 完成单个任务
REM   progress.bat check phase1            - 检查依赖
REM   progress.bat note "message"          - 添加备注

cd /d "%~dp0"

if "%1"=="read" (
    python progress_manager.py read
) else if "%1"=="start" (
    python progress_manager.py start %2
) else if "%1"=="complete" (
    python progress_manager.py complete %2
) else if "%1"=="task" (
    python progress_manager.py task %2 %3
) else if "%1"=="check" (
    python progress_manager.py check %2
) else if "%1"=="note" (
    python progress_manager.py note %2 %3 %4 %5 %6 %7 %8 %9
) else (
    echo FormHelper 进度管理工具
    echo.
    echo 用法:
    echo   progress.bat read                    - 读取当前进度
    echo   progress.bat start phase1            - 开始阶段1
    echo   progress.bat complete phase1         - 完成阶段1
    echo   progress.bat task phase1 task_name   - 完成单个任务
    echo   progress.bat check phase1            - 检查依赖
    echo   progress.bat note "message"          - 添加备注
)

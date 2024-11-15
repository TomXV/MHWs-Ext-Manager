@setlocal enableextensions
@pushd %~dp0
@echo off

:: リストファイルの確認
set LISTFILE=MHWs_STM_Beta.list
if not exist "%LISTFILE%" (
    echo %LISTFILE% is missing. Downloading it from GitHub...
    powershell -Command "wget https://raw.githubusercontent.com/Ekey/REE.PAK.Tool/main/Projects/%LISTFILE% -OutFile %LISTFILE%"
    if not exist "%LISTFILE%" (
        echo Error: Failed to download %LISTFILE%. Exiting.
        pause
        exit /b
    )
) else (
    echo %LISTFILE% found.
)

:: RETool.exe の確認
if not exist "RETool.exe" (
    echo RETool is missing. Downloading it from GitHub...
    powershell -Command "wget https://raw.githubusercontent.com/mhvuze/MonsterHunterRiseModding/main/files/REtool.exe -OutFile RETool.exe"
    if not exist "RETool.exe" (
        echo Error: Failed to download RETool.exe. Exiting.
        pause
        exit /b
    )
) else (
    echo RETool.exe found.
)

:: PAK ファイルの確認
if "%~1"=="" (
    echo Drag a .pak file from Monster Hunter Wilds onto this script to extract it.
    pause
    exit /b
)

:: RETool.exe で展開
echo Running RETool.exe on %1...
.\RETool.exe -h %LISTFILE% -x -skipUnknowns %1
@REM if errorlevel 1 (
@REM     echo Error: RETool.exe execution failed. Exiting.
@REM     pause
@REM     exit /b
@REM )

@setlocal enabledelayedexpansion

:: Monster Hunter Wilds の PAK かどうか確認
set SRC_FOLDER=.\re_chunk_000.pak.sub_000
set DST_FOLDER=.\re_chunk_000
set RENAMED_FOLDER=.\re_chunk_000(merged)

if exist "%DST_FOLDER%" if exist "%SRC_FOLDER%" (
    echo Guess: Monster Hunter Wilds?

    :input_prompt
    set /p confirm="Do you want to merge the extracted files? [y/n]: "

    :: 入力が空の場合、再入力を求める
    if "!confirm!"=="" (
        echo Input cannot be empty. Please enter [y/n]
        goto input_prompt
    )

    :: 有効な入力のみ処理
    if /i "!confirm!"=="y" (
        echo Merging files...
        echo Running: MHWs-Ext-Manager.exe merge "%SRC_FOLDER%" "%DST_FOLDER%"
        .\MHWs-Ext-Manager.exe merge "%SRC_FOLDER%" "%DST_FOLDER%"
        if errorlevel 1 (
            echo Error: Merge operation failed. Exiting.
            pause
            exit /b
        )

        :: フォルダをリネーム
        echo Running: rename "%SRC_FOLDER%" "%RENAMED_FOLDER%"
        move "%DST_FOLDER%" "%RENAMED_FOLDER%"
        if errorlevel 1 (
            echo Error: Failed to rename "%DST_FOLDER%" to "%RENAMED_FOLDER%"
            pause
            exit /b
        ) else (
            echo Renamed "%DST_FOLDER%" to "%RENAMED_FOLDER%"
            rmdir /S /Q "%SRC_FOLDER%"
            echo Removed "%SRC_FOLDER%"
            pause
            exit /b
        )
    ) else if /i "!confirm!"=="n" (
        echo Merge operation cancelled.
        pause
        exit /b
    ) else (
        echo Invalid input. Please enter [y/n]
        goto input_prompt
    )
) else (
    echo Either "%DST_FOLDER%" or "%SRC_FOLDER%" does not exist. No merge performed.
    pause
    exit /b
)

@popd
@pause

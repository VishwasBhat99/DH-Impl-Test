@echo off

set ERROR_LOG=%SH_LOGS%\BA\%3\%1\ba-loader-log.txt
if exist %ERROR_LOG% del %ERROR_LOG%

echo ---------------Deleting Data from Tables--------------- >> %ERROR_LOG%
call %BASE_PATH%\scripts\loader-scripts\BA\delete.bat %1 %2 %3 2>&1 
if %ERRORLEVEL% neq 0 echo "Calling %BASE_PATH%\scripts\loader-scripts\BA\delete.bat failed." >> %ERROR_LOG% & exit /b %ERRORLEVEL%

echo ---------------Exporting BALM LLGDef Data from BALM Tables--------------- >> %ERROR_LOG%
call %BASE_PATH%\scripts\loader-scripts\BA\export_llgdef.bat %1 %2 %3 2>&1 
if %ERRORLEVEL% neq 0 echo "Calling %BASE_PATH%\scripts\loader-scripts\BA\export_llgdef.bat failed." >> %ERROR_LOG% & exit /b %ERRORLEVEL%

echo ---------------Loading BALM LLGDef Data into BA Tables--------------- >> %ERROR_LOG%
call %BASE_PATH%\scripts\loader-scripts\BA\load_llgdef.bat %1 %2 %3 2>&1 
if %ERRORLEVEL% neq 0 echo "Calling %BASE_PATH%\scripts\loader-scripts\BA\load_llgdef.bat failed." >> %ERROR_LOG% & exit /b %ERRORLEVEL%

echo ---------------Checking if %2 is a holiday--------------- >> %ERROR_LOG%
set HOLIDAY_LIST=%SH_RESOURCES%/OPERATIONAL-FILES/BA-FILES/holiday.txt
set is_holiday=false

for /f "usebackq delims=" %%a in (%HOLIDAY_LIST%) do (
    if "%2"=="%%a" (
        set is_holiday=true
        goto :break_loop
    )
)

:break_loop
if "%is_holiday%"=="true" (
    echo It is a holiday on %2. Data not processed.
    echo It is a holiday on %2. Data not processed. >> %ERROR_LOG%
    goto :end
) else (
    echo It is not a holiday on %2 Processing BA data.
    echo It is not a holiday on %2 Processing BA data. >> %ERROR_LOG%

    echo ----------Extracting BALM Data----------- >>%ERROR_LOG%
    call %BASE_PATH%\scripts\loader-scripts\BA\export_balm_data.bat %1 %2 %3 2>&1
    if %ERRORLEVEL% neq 0 (
	echo "Calling %BASE_PATH%\scripts\loader-scripts\BA\export_balm_data.bat failed." >> %ERROR_LOG%
	exit /b %ERRORLEVEL%
)

    echo ----------Loading BALM Data into BA table.---------- >> %ERROR_LOG%
    call %BASE_PATH%\scripts\loader-scripts\BA\load_balm_data.bat %1 %2 %3 2>&1 
    if %ERRORLEVEL% neq 0 echo "Calling %BASE_PATH%\scripts\loader-scripts\BA\load_balm_data.bat failed." >> %ERROR_LOG% & exit /b %ERRORLEVEL%

    echo ----------Loading BA Data into BA table.---------- >> %ERROR_LOG%
    call %BAS_PATH%\scripts\loader-scripts\BA\load_ba_data.bat %1 %2 %3 2>&1 
    if %ERRORLEVEL% neq 0 echo "Calling %BASE_PATH%\scripts\loader-scripts\BA\load_ba_data.bat failed." >> %ERROR_LOG% & exit /b %ERRORLEVEL%
)

:end
echo ----------BA loader processed.---------- >> %ERROR_LOG%

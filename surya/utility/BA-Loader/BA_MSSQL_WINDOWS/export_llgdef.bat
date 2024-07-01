@echo off

set FILE_NAME="%SH_SUMMARY%\BA\%3\%1\balm_llgdef.txt"
if exist %FILE_NAME% del %FILE_NAME%

set ERROR_LOG=%SH_LOGS%\BA\%3\%1\ba-loader-log.txt

bcp "select '%3' as CCYID, ISNULL(LLGId, 'NA')as LLGId,ISNULL(LLGDesc, 'NA') as LLGDesc from LLGDef;" queryout "%FILE_NAME%" -S %BALM_DBSERVER% -U %BALM_USER% -P %BALM_PASS% -d %BALM_DBNAME% -c -t "|"2>&1
if %ERRORLEVEL% neq 0 echo "Bulk Export of data from LLGDef Failed." >> %ERROR_LOG% & exit /b %ERRORLEVEL%



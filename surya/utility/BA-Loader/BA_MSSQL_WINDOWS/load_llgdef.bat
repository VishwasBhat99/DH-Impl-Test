@ECHO OFF

set ERROR_LOG=%SH_LOGS%\BA\%3\%1\ba-loader-log.txt
set FILE_NAME=%SH_SUMMARY%\BA\%3\%1\balm_llgdef.txt

sqlcmd -S %BA_DBSERVER% -d %BA_DBNAME% -U %BA_USER% -P %BA_PASS% -V 1 -I -Q "bulk insert BALMProductDef from '%FILE_NAME%' with(fieldterminator='|',rowterminator='0x0a')"2>&1
if %ERRORLEVEL% neq 0 echo "Loading BALM LLGDef data into BALMProductDef table failed." >> %ERROR_LOG% & exit /b %ERRORLEVEL%

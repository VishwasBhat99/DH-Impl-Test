@echo off

set ERROR_LOG=%SH_LOGS%\BA\%3\%1\ba-loader-log.txt

sqlcmd -S %BA_DBSERVER% -d %BA_DBNAME% -U %BA_USER% -P %BA_PASS% -V 1 -I -Q "DELETE FROM BALLG_MBTotals  WHERE AsOn = convert(datetime,'%2',105)"2>&1
if %ERRORLEVEL% neq 0 echo "Deletion of Data from BALLG_MBTotals failed." >> %ERROR_LOG% & exit /b %ERRORLEVEL%

sqlcmd -S %BA_DBSERVER% -d %BA_DBNAME% -U %BA_USER% -P %BA_PASS% -V 1 -I -Q "DELETE FROM BALMProductDef  WHERE CountryID = '%3'"2>&1 
if %ERRORLEVEL% neq 0 echo "Deletion of Data from BALMProductDef failed." >> %ERROR_LOG% & exit /b %ERRORLEVEL%

sqlcmd -S %BA_DBSERVER% -d %BA_DBNAME% -U %BA_USER% -P %BA_PASS% -V 1 -I -Q "DELETE FROM BALMInputTotals  WHERE CountryID = '%3' and AsOnDt='%2'"2>&1
if %ERRORLEVEL% neq 0 echo "Deletion of Data from BALMInputTotals failed." >> %ERROR_LOG% & exit /b %ERRORLEVEL%

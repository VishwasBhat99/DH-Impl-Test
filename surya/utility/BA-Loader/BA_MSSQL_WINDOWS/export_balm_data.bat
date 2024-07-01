@echo off

set FILE_NAME=%SH_SUMMARY%\BA\%3\%1\balm_data.txt
set ERROR_LOG=%SH_LOGS%/BA\%3\%1\ba-loader-log.txt

bcp "select '%3' as ccyid,ISNULL(SubType_ID, 'NA') ,ISNULL(As_On, 'NA'), ISNULL(Currency_ID, 'NA'), ISNULL(SLRorIRS, 'NA'), ISNULL(SchemeID, 'NA'),ISNULL(CashflowType, 'NA') ,ISNULL(Amount, 0),ISNULL(InterestRate, 0) from tblProductTotals  where As_On='%2';" queryout %FILE_NAME% -S %BALM_DBSERVER% -U %BALM_USER% -P %BALM_PASS% -d %BALM_DBNAME% -c -t "|"2>&1
if %ERRORLEVEL% neq 0 echo "Bulk Export of data from tblProductTotals failed." >> %ERROR_LOG% & exit /b %ERRORLEVEL%



@echo off 

SET ASONDATE=%1
SET LOG_FILE="log-check.txt"
SET DIAGNOSTICS_FILE="diag-log-check.txt"
SET CONNECTION_STRING="BALM4DEMO01|yyv3fSz5KU|10.10.10.12:1521/orcl.corporate.com"


C:\Users\secadmin\Desktop\test\upload_check.exe  --log-file %LOG_FILE%  --log-level debug --diagnostics-log-file %DIAGNOSTICS_FILE%  --as-on-date %ASONDATE% --connection-string %CONNECTION_STRING%

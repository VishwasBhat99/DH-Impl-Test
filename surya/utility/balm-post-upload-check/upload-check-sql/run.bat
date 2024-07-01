@echo off 

SET ASONDATE=%1
SET LOG_FILE="C:\Users\rmvpn1\Desktop\upload-check\log-check.txt"
SET DIAGNOSTICS_FILE="C:\Users\rmvpn1\Desktop\upload-check\diag-log-check.txt"
SET CONNECTION_STRING="Driver={ODBC Driver 11 for SQL Server};Server=192.168.66.13;Database=BALM4_NBS;UID=balm;PWD=balm321;"

C:\Users\rmvpn1\Desktop\upload-check\upload_check.exe  --log-file %LOG_FILE%  --log-level debug --diagnostics-log-file %DIAGNOSTICS_FILE%  --as-on-date %ASONDATE% --connection-string %CONNECTION_STRING%

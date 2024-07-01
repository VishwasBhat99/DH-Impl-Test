@echo off

@REM Path to which scenarios need to be downloaded
set /p base_path="Enter base file path: "
set scenario_path="%base_path%\scenarios"
mkdir %scenario_path%

@REM Command to download stream id's to file
BCP "SELECT STREAMID as id FROM RUNCONTROL_NAME.dbo.STREAMDEF" QUERYOUT %base_path%\streamids.txt -c -t, -S DB_CONNECTION_NAME -U RUNCONTROL_USERNAME -P RUNCONTROL_PASSWORD -t "|"

@REM Command to download scenarios to folder
FOR /F  %%x in (%base_path%\streamids.txt) DO  BCP "SELECT STREAMDESC FROM RUNCONTROL_NAME.dbo.STREAMDEF where STREAMID=%%x" QUERYOUT %scenario_path%\%%x.json -f bcp.fmt -S DB_CONNECTION_NAME -U RUNCONTROL_USERNAME -P RUNCONTROL_PASSWORD

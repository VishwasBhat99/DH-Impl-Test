@echo off

set ERROR_LOG=%SH_LOGS%\BA\%3\%1\ba-loader-log.txt

sqlcmd -S %BA_DBSERVER% -d %BA_DBNAME% -U %BA_USER% -P %BA_PASS% -V 1 -I -Q "insert  into BALLG_MBTotals (LLGID,AsOn,CurrencyID,DIM1,DIM2,DIM3,DIM4,DIM5,Amount,IR) (select b.BALLGID,convert(datetime,t.AsOnDt,105) as AsOnDt,t.CcyID,'DIM1','DIM2','DIM3','DIM4','DIM5',Sum(t.Amount) as Amount,ISNULL(Sum(InterestRate*Amount)/NULLIF(SUM(Amount),0),0) as AvgIR from (select BALLGID, max(EffFromDt) as MaxEffDt from BALLGtoBALMLLGMap where EffFromDt <= convert(datetime,'%2',105) group by BALLGID) a, BALLGtoBALMLLGMap b, BALMInputTotals t where a.BALLGID=b.BALLGID and a.MaxEffDt=b.EffFromDt  and b.BALMLLGID=t.BALMLLGID and t.AsOnDt='%2' and t.SLRorIRS in ('SLR','ALL') and t.CountryID=b.BALMCountryID  group by b.BALLGID,t.CcyID,t.AsOnDt)" 2>&1
if %ERRORLEVEL% neq 0 echo "Loading of BA data into BALLG_MBTotals failed." >> %ERROR_LOG% & exit /b %ERRORLEVEL%

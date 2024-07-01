master=$( ls -t ../FF_FN_ALM_LOAN_EXTRACT_MOR_* | head -1 )
cashflows=$( ls -t ../FF_FN_ALM_LOAN_SCHEDULE_CF_* | head -1 )
sqlldr balmusr/HdFcBank13\$\# control=./Fin_Loans_Master.ctl data=$master LOG=$ORACLE_HOME/ImportLog/Fin_Loans_Master.log BAD=$ORACLE_HOME/ImportLog/Fin_Loans_Master.BAD
sqlldr balmusr/HdFcBank13\$\# control=./Fin_Loans_Cashflows.ctl data=$cashflows LOG=$ORACLE_HOME/ImportLog/Fin_Loans_Cashflows.log BAD=$ORACLE_HOME/ImportLog/Fin_Loans_Cashflows.BAD

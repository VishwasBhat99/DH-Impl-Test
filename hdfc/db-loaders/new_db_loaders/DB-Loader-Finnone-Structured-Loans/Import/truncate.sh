#!/usr/bin/env bash

sqlplus -s $CON_STR_IND << EOF

truncate table Fin_Loans_Master;
truncate table Fin_Loans_Cashflows;

exit
EOF

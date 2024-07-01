#!/usr/bin/env bash

sqlplus -s $CON_STR_IND << EOF

truncate table SEC_INVST_MASTER;
truncate table SEC_INVST_CASHFLOW;

exit
EOF

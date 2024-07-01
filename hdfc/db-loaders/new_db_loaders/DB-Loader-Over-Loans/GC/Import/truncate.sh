#!/usr/bin/env bash


sqlplus -s $CON_STR_GC << EOF

truncate table OVER_LOANS_MASTER;
truncate table OVER_LOANS_CASHFLOW;

exit
EOF

#!/usr/bin/env bash

sqlplus -s $CON_STR_GC << EOF

truncate table MONEY_MARKET_MASTER;
truncate table MONEY_MARKET_CASHFLOW;

exit
EOF

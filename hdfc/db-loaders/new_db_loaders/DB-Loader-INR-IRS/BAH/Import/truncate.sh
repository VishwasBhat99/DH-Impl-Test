#!/usr/bin/env bash

sqlplus -s $CON_STR_BH << EOF

truncate table INR_IRS_SWAP_MASTER;

exit
EOF
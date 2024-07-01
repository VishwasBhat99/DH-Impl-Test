#!/usr/bin/env bash

sqlplus -s $CON_STR_IND << EOF

truncate table OPTION_MASTER;

exit
EOF

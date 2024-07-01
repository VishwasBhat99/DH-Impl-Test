#!/usr/bin/env bash

sqlplus -s $CON_STR_BH << EOF

truncate table OPTION_MASTER;

exit
EOF

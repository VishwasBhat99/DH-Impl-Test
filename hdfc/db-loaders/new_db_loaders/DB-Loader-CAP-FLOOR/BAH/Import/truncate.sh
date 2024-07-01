#!/usr/bin/env bash

sqlplus -s $CON_STR_BH << EOF

truncate table CAP_FLOOR_MASTER;

exit
EOF

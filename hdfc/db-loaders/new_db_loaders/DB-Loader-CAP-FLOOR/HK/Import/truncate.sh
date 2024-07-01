#!/usr/bin/env bash

sqlplus -s $CON_STR_HK << EOF

truncate table CAP_FLOOR_MASTER;

exit
EOF

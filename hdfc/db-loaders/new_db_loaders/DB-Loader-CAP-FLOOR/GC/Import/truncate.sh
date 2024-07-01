#!/usr/bin/env bash

sqlplus -s $CON_STR_GC << EOF

truncate table CAP_FLOOR_MASTER;

exit
EOF

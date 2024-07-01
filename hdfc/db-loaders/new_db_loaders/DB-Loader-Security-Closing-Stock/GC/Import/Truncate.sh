#!/usr/bin/env bash

sqlplus -s $CON_STR_GC << EOF

truncate table Sec_Close_Stock;

exit
EOF
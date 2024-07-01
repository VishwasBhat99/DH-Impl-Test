#!/usr/bin/env bash

sqlplus -s $CON_STR_IND << EOF

truncate table Sec_Close_Stock;

exit
EOF
#!/usr/bin/env bash

sqlplus -s $CON_STR_BH << EOF

truncate table Sec_Close_Stock;

exit
EOF
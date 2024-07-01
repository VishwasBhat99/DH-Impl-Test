#!/usr/bin/env bash

sqlplus -s $CON_STR_HK << EOF

truncate table Sec_Close_Stock;

exit
EOF
#!/usr/bin/env bash

read as_on_dt < ../../common_resources.txt

sqlplus -s balmusr/HdFcBank13\$\# << EOF

@delete_TTL.sql $as_on_dt

exit
EOF

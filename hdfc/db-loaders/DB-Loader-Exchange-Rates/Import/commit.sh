#!/usr/bin/env bash

sqlplus -s balmusr/HdFcBank13\$\# << EOF

@commit.sql

exit
EOF

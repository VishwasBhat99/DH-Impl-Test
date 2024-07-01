#!/usr/bin/env bash

sqlplus -s $CON_STR_BAUSR << EOF

commit;
exit;
EOF

#!/usr/bin/env bash

sqlplus -s $CON_STR_BAUSR << EOF

DELETE FROM "BALLG_MBTotals"  WHERE "AsOn" =TO_DATE('$2','DD-MM-YYYY');
DELETE FROM "BALMProductDef"  WHERE "CountryID" = '$3';
DELETE FROM "BALMInputTotals"  WHERE "CountryID" = '$3' and "AsOnDt"='$2';

commit;
exit;
EOF

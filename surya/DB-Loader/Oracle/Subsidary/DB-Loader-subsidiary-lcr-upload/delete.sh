#!/usr/bin/env bash

read ason_date < ../../common-resources/ason_date.txt
#read country_code < ../../common-resources/country_code.txt
#read currency_id < ../../common-resources/currency_id.txt

sqlplus -s $CON_STR_BALMUSR << EOF
@delete_subsidiary_lcr.sql $ason_date 
exit
EOF

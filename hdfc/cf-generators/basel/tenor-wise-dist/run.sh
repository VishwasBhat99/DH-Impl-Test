#!/usr/bin/env bash

RET_CUST_AGGR=$"test-bed/ret-input.txt"
NON_RET_CUST_AGGR=$"test-bed/non-ret-input.txt"
OUTPUT=$"test-bed/output"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
WD_RET_INPUT=$"test-bed/wd-ret-input.txt"
WD_NON_RET_INPUT=$"test-bed/wd-non-ret-input.txt"
NWD_RET_INPUT=$"test-bed/nwd-ret-input.txt"
NWD_NON_RET_INPUT=$"test-bed/nwd-non-ret-input.txt"
WD_CUST=$"test-bed/wd_cust.txt"
NWD_CUST=$"test-bed/nwd_cust.txt"

cut -d "|" -f 2- ${WD_RET_INPUT} > ${WD_CUST}
cut -d "|" -f 2- ${NWD_RET_INPUT} > ${NWD_CUST}
cat ${WD_NON_RET_INPUT} >> ${WD_CUST}
cat ${NWD_NON_RET_INPUT} >> ${NWD_CUST}

cargo run -- \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--ret-cust-aggr-file ${RET_CUST_AGGR} \
--non-ret-cust-aggr-file ${NON_RET_CUST_AGGR} \
--wd-cust-file ${WD_CUST} \
--nwd-cust-file ${NWD_CUST} \
--as-on-date 31-08-2021 
#--log-level trace \
#--diagnostics-flag true

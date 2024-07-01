#!/usr/bin/env bash

FTPRUNID=$"110228"
FROM_DATE=$"31-01-2019"
TO_DATE=$"31-01-2019"
INPUT=$"input/CFOutput_dep.cf"
METADATA=$"input/metadata_dep.json"
AGGR_FILE=$"input/dep_aggr.txt"
AMB_FILE=$"input/amb-rd.csv"
EXCH_RATE_FILE=$"input/Exch.txt"
OUTPUT=$"output/FTPCFOutput_TD"
LOG_FILE=$"output/log.txt"
DIAGNOSTICS_FILE=$"output/diag-log.txt"

cargo run --release -- \
--ftp-runid ${FTPRUNID} \
--from-date ${FROM_DATE} \
--to-date ${TO_DATE} \
--input-file ${INPUT} \
--meta-data-file ${METADATA} \
--aggr-file-path ${AGGR_FILE} \
--amb-file-path ${AMB_FILE} \
--output-file ${OUTPUT} \
--exch-rate-file ${EXCH_RATE_FILE} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false

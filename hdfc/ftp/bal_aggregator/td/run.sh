#!/usr/bin/env bash

FROM_DATE=$"01-01-2019"
TO_DATE=$"31-01-2019"
INPUT=$"input/dep.cf"
METADATA=$"input/metadata_dep.json"
OUTPUT=$"output/dep_aggr.txt"
LOG_FILE=$"output/log.txt"
DIAGNOSTICS_FILE=$"output/diag-log.txt"

cargo run --release -- \
--from-date ${FROM_DATE} \
--to-date ${TO_DATE} \
--input-file ${INPUT} \
--meta-data-file ${METADATA} \
--output-file ${OUTPUT} \
--matched-term-lock false \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false

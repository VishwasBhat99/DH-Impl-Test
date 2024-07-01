#!/usr/bin/env bash

LOG_FILE=$"log.txt"
DIAGNOSTICS_FILE=$"diag-log.txt"
COUNTRY_ID=$"IND"
DBCONFIG=$"dbconfig.json"
OUTPUT_PATH=$"/test-bed"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--country-id ${COUNTRY_ID} \
--output-path ${OUTPUT_PATH} \
--dbconfig-file ${DBCONFIG} \
--log-level trace \
--diagnostics-flag false

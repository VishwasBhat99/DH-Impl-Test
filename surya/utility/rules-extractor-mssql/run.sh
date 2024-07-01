#!/usr/bin/env bash

LOG_FILE=$"log.txt"
DIAGNOSTICS_FILE=$"diag-log.txt"
COUNTRY_ID=$"IND"
OUTPUT_FILE_PATH=$"output.txt"

cargo run -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--country-id ${COUNTRY_ID} \
--output-file-path ${OUTPUT_FILE_PATH} \
--connection-string "Driver={ODBC Driver 17 for SQL Server};Server={DEVDB\SQLSERVER2017};Database={IA4_DEV};UID={IA};PWD={Z1BsbT2TwWU0};" \
--log-level trace \
--diagnostics-flag false

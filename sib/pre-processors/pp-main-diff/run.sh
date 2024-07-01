#!/usr/bin/env bash
MAPPING_MASTER=$"test-bed/mapping_master.txt"
RECON=$"test-bed/recon.txt"
GSTT=$"test-bed/gstt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OUTPUT_FILE=$"test-bed/output.txt"

cargo run --release -- \
--gstt-extraction-file ${GSTT} \
--mapping-file-path ${MAPPING_MASTER} \
--recon-file ${RECON} \
--output-file ${OUTPUT_FILE} \
--as-on-date  30-04-2023 \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false

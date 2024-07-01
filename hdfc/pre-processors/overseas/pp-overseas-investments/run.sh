#!/usr/bin/env bash

INPUT=$"test-bed/GC/hst3053_gft20190227230530360679.xls"
OUTPUT=$"test-bed/output.txt"
RECOUT=$"test-bed/GCOverInvstReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file ${INPUT} \
--output-file ${OUTPUT} \
--rec-output-file ${RECOUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--sheet-name Sheet1 \
--gl-type GC-Over-Invst \
--input-file-name GC_COUNTRY_HST3053 \
--log-level trace \
--diagnostics-flag true

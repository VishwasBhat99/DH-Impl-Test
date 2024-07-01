#!/usr/bin/env bash

INPUT=$"test-bed/NPATEST1.txt"
REF1=$"test-bed/MappingMaster_v2.0.xlsx"
REF2=$"test-bed/npa.txt"
OUTPUT=$"test-bed/output.txt"
CONCAT=$"test-bed/concat.txt"
RECOUT=$"test-bed/ODReconRpt.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--input-file-path ${INPUT} \
--alm-master ${REF1} \
--npa-consolidated ${REF2} \
--alm-master-sheet-name "Sheet1" \
--npa-sheet-name "NPA Consolidated Statements" \
--output-file-path ${OUTPUT} \
--rec-output-file-path ${RECOUT} \
--concat-file-path ${CONCAT} \
--log-file-path ${LOG_FILE} \
--diagnostics-file-path ${DIAGNOSTICS_FILE} \
--as-on-date 27-01-2019 \
--log-level trace \
--diagnostics-flag true

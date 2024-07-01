#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
FAE=$"test-bed/FAE.TXT"
FBH=$"test-bed/FBH.TXT"
FEI=$"test-bed/FEI.TXT"
IDT=$"test-bed/IDT.TXT"
TFAT=$"test-bed/TFAT.TXT"
INPUT=$"test-bed/INPUT.TXT"
OUTPUT_FILE=$"test-bed/output.txt"

cargo run -- \
--as-on-date 30-06-2023 \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--fae-file-path ${FAE} \
--fbh-file-path ${FBH} \
--fei-file-path ${FEI} \
--idt-file-path ${IDT} \
--input-file-path ${INPUT} \
--log-file ${LOG_FILE} \
--output-file ${OUTPUT_FILE} \
--tfat-file-path ${TFAT} 
#--log-level trace \
#--diagnostics-flag false

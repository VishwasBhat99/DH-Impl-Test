#!/usr/bin/env bash

INPUT_FILE=$"test-bed/TB_Input_file.txt"
ALM=$"test-bed/alm_master.xlsx"
FIN=$"test-bed/FinCodeMap.xlsx"
GLEX=$"test-bed/GlExcludeMaster.txt"
MOC=$"test-bed/MOC_File.xlsx"
OUTPUT=$"test-bed/op/output"
LOG_FILE=$"test-bed/op/log.txt"
DIAGNOSTICS_FILE=$"test-bed/op/diag-log.txt"

cargo run  -- \
--input-file ${INPUT_FILE} \
--alm-master-file ${ALM} \
--fin-code-map-file ${FIN} \
--gl-ex-master ${GLEX} \
--gl-moc-file ${MOC} \
--alm-master-sheet-name Sheet1 \
--fin-code-sheet-name Sheet1 \
--gl-moc-sheet-name Sheet1 \
--currency INR \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-12-2021 \
#--log-level trace \
#--diagnostics-flag true

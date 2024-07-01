#!/usr/bin/env bash

INPUT_FILE=$"test-bed/TB_Input_file.txt"
REF1=$"test-bed/alm_master.xlsx"
REF2=$"test-bed/FinCodeMap.xlsx"
REF3=$"test-bed/GlExcludeMaster.txt"
REF4=$"test-bed/MOC_File.xlsx"
OUTPUT=$"test-bed/op/output"
LOG_FILE=$"test-bed/op/log.txt"
DIAGNOSTICS_FILE=$"test-bed/op/diag-log.txt"

cargo run  -- \
--input-file ${INPUT_FILE} \
--ref-file-1 ${REF1} \
--fin-code-map-file ${REF2} \
--gl-ex-master ${REF3} \
--gl-moc-file ${REF4} \
--gl-moc-sheet-name Sheet1 \
--alm-master-sheet-name Sheet1 \
--fin-code-sheet-name Sheet1 \
--currency INR \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-2019 \
--log-level trace \
--diagnostics-flag true

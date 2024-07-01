#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
COST_DATA_PP1_INPUT_FILE=$"test-bed/cost-data-PP1.txt"
SOLDIM_INPUT_FILE=$"test-bed/solDim-file.txt"
DIVDIM_INPUT_FILE=$"test-bed/divDim-file.txt"
PRDDIM_INPUT_FILE=$"test-bed/prdDim-file.txt"
COSTALLOCPP2_OUTPUT_FILE=$"test-bed/costAlloc-PP2-file.txt"


cargo run --release -- \
--cost-data-pp1-file ${COST_DATA_PP1_INPUT_FILE} \
--soldim-file ${SOLDIM_INPUT_FILE} \
--divdim-file ${DIVDIM_INPUT_FILE} \
--prddim-file ${PRDDIM_INPUT_FILE} \
--cost-alloc-pp2-file ${COSTALLOCPP2_OUTPUT_FILE} \
--as-on-date  11-07-2022 \
--unique-id-columns "2,4,10" \
--delimeter "|" \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
#--log-level trace \
#--diagnostics-flag false
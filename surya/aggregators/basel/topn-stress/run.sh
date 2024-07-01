#!/usr/bin/env bash

CUST_AGGR_RET=$"test-bed/cust-aggr-ret-ccy.txt"
CUST_AGGR_NONRET=$"test-bed/cust-aggr-nonret-ccy.txt"
CLASS_LLG_MAPPER=$"test-bed/class-llg-mapper.txt"
OUTPUT=$"test-bed/output.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diaglog.txt"
TOPN_DEP=$"test-bed/topn-dep.txt"
EXRT=$"test-bed/exrt.txt"

cargo run --release -- \
--retail-input-file ${CUST_AGGR_RET} \
--non-retail-input-file ${CUST_AGGR_NONRET} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--class-llg-mapper-file ${CLASS_LLG_MAPPER} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--topn-dep-file ${TOPN_DEP} \
--exrt-rate-file ${EXRT} \
--ccy-id INR \
--country-id IND \
--as-on-date 31-01-2023 \
--default-llg 10000 \
--log-level trace \
--diagnostics-flag true

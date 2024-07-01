#!/usr/bin/env bash

INCOME_MASTER=$"test-bed/INCOME_MASTER.txt"
AGGR_FILE=$"test-bed/od_aggr_op.txt"
COMMON_CAP=$"test-bed/od-common_cap_op.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run  -- \
--income-master-file ${INCOME_MASTER} \
--aggr-input-file ${AGGR_FILE} \
--common-cap-file ${COMMON_CAP} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--as-on-date 31-01-2023 \
#--log-level trace \
#--diagnostics-flag true

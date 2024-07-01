#!/usr/bin/env bash

CASATD=$"test-bed/CASATDMaster_30062020.txt"
ADV=$"test-bed/AdvanceMaster_30062020.txt"
OPS=$"test-bed/OPSAccData.txt"
MUL_DEPO_FILE=$"test-bed/mult_depo.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"
OUTPUT=$"test-bed/output.txt"
PROD_CODE_FILE=$"test-bed/prod-code.txt"
CONST_DESC_FILE=$"test-bed/const-desc.txt"
cargo run --release -- \
--casatd-master "${CASATD}" \
--output-file-path "${OUTPUT}" \
--mult-depo-file ${MUL_DEPO_FILE} \
--log-file-path "${LOG_FILE}" \
--diagnostics-file-path "${DIAGNOSTICS_FILE}" \
--adv-master "${ADV}" \
--ops-acc-data "${OPS}" \
--const-desc-file-path "${CONST_DESC_FILE}" \
--prod-code-file-path "${PROD_CODE_FILE}" \
--log-level trace \
--diagnostics-flag true

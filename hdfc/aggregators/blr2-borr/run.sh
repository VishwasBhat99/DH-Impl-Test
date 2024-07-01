#!/usr/bin/env bash

CCM=$"../input-resources/cust-master.txt"
OUTPUT=$"../output/borr.txt"
LOG_FILE=$"../output/log.txt"
DIAGNOSTICS_FILE=$"../output/diag-log.txt"
CONFIG=$"../input-resources/borr_config.json"

dt < ../../../common_resources.txt

cargo run --release -- \
--country-id INDIA \
--base-ccy INR \
--cust-code-master ${CCM} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--config_file_path ${CONFIG} \
--as-on-date $dt \
#--log-level trace \
#--diagnostics-flag true

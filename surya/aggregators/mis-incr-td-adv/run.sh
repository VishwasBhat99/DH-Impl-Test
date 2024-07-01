#!/usr/bin/env bash

OUTPUT=$"test-bed/summary_Advances_Finnone_alm.txt"
LOG_FILE=$"test-bed/log-advances-finnone-incr_$3.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log-advances-finnone-incr_$3.txt"
CONFIG_FILE=$"test-bed/config_Advances_incr_finnone_alm-test.json"

cargo run -- \
--report-id RPT1406 \
--home-currency INR \
--config-file $CONFIG_FILE \
--output-file $OUTPUT \
--log-file $LOG_FILE \
--diagnostics-log-file $DIAGNOSTICS_FILE \
--as-on-date 31-12-2020 \
#--log-level debug \
#--diagnostics-flag true \
#--is-consolidated true \
#--exchange-rate-file $CURRENCY_CONV_FILE \

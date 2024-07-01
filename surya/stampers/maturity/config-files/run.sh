#!/usr/bin/env bash

FROM_DATE=$""
TO_DATE=$""
CONFIG=$""
M_RULE=$""
BC_RULE=$""
ADJ_RULE_FIX=$""
ADJ_RULE_VAR=$""
SPREAD=$""
ADJ_RATES=$""
OUTPUT=$""
METHOD=$""
BAL_SLAB=$""
BC_FILE=$""
LOG_FILE=$""
DIAGNOSTICS_FILE=$""

cargo run --release -- \
    --from-date ${FROM_DATE} \
    --to-date ${TO_DATE} \
    --config-file ${CONFIG} \
    --method-rules-file ${M_RULE} \
    --bc-rule-file ${BC_RULE} \
    --fix-adj-rule-file ${ADJ_RULE_FIX} \
    --var-adj-rule-file ${ADJ_RULE_VAR} \
    --output-file ${OUTPUT} \
    --bc-file ${BC_FILE} \
    --spread-file ${SPREAD} \
    --adj-rates-file ${ADJ_RATES} \
    --method-req-fields-file-path ${METHOD} \
    --default-method 1002 \
    --default-basecurve 1110 \
    --fixed-adjustments-count 3 \
    --var-adjustments-count 3 \
    --log-file ${LOG_FILE} \
    --diagnostics-log-file ${DIAGNOSTICS_FILE} \
    --log-level trace \
    --bal-prec 4 \
    --rate-prec 4 \
    --diagnostics-flag true \
    --ccy INR \
    --is-def-from-date true \
    --bal-slab ${BAL_SLAB}

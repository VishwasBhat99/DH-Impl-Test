CONFIG=$"test-bed/TEST_BED/test_bed/ftp-borrowing-config.json"
M_RULE=$"test-bed/TEST_BED/test_bed/borrowing-method-rules_test.txt"
BC_RULE=$"test-bed/TEST_BED/test_bed/borrowing-bc-rules_test.txt"
LLG_RULE=$"test-bed/TEST_BED/test_bed/borrowing-llg-rules_test.txt"
DEF_LLG_RULE=$"test-bed/TEST_BED/test_bed/borrowing-llg-rules_test.txt"
DEF_METHOD_RULE=$"test-bed/TEST_BED/test_bed/def-borrowing-method-rules_test.txt"
DEF_BC_RULE=$"test-bed/TEST_BED/test_bed/def-borrowing-bc-rules_test.txt"
AORL_RULE=$"test-bed/TEST_BED/test_bed/def-borrowing-aorl-rules_test.txt"
ADJ_RULE_FIX=$"test-bed/TEST_BED/test_bed/borrowing-fix-adj-rules_test.txt"
ADJ_RULE_VAR=$"test-bed/TEST_BED/test_bed/borrowing-var-adj-rules_test.txt"
SPREAD=$"test-bed/TEST_BED/test_bed/borrowing-spread_test.txt"
ADJ_RATES=$"test-bed/TEST_BED/test_bed/adj_rates.txt"
OUTPUT=$"test-bed/TEST_BED/test_bed/BORROWING-STAMPER-output.txt"
METHOD=$"test-bed/TEST_BED/test_bed/borrowing-method-req-fields.json"
BAL_SLAB=$"test-bed/TEST_BED/test_bed/BL-bal-slab_test.txt"
BC_FILE=$"test-bed/TEST_BED/test_bed/BMRates/"
LOG_FILE=$"test-bed/TEST_BED/test_bed/brrowing-log.txt"
DIAGNOSTICS_FILE=$"test-bed/TEST_BED/test_bed/borrowing-diag-log.txt"

cargo run --release -- \
--adj-rates-file ${ADJ_RATES} \
--bal-slab ${BAL_SLAB} \
--bc-file ${BC_FILE} \
--bc-rule-file ${BC_RULE} \
--llg-rule-file ${LLG_RULE} \
--ccy INR \
--config-file ${CONFIG} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--fix-adj-rule-file ${ADJ_RULE_FIX} \
--fixed-adjustments-count 3 \
--log-file ${LOG_FILE} \
--method-req-fields-file-path ${METHOD} \
--method-rules-file ${M_RULE} \
--output-file ${OUTPUT} \
--spread-file ${SPREAD} \
--to-date 31-03-2024 \
--var-adjustments-count 3 \
--var-adj-rule-file ${ADJ_RULE_VAR} \
--bal-prec 8 \
--rate-prec 3 \
--diagnostics-flag false \
--is-def-from-date true \
--apply-base-curve-2 false \
--is-absolute true \
--rate-flag "" \
--default-aorl-flag "L" \
--aorl-rule-file ${AORL_RULE} \
--skip-amb-header false \
--is-extrapolation-required false \
--day-count-basis ACTby365 \
--default-llg-file $DEF_LLG_RULE \
--default-method-file ${DEF_METHOD_RULE} \
--default-basecurve-file ${DEF_BC_RULE} \
--bmrate-accuracy "D" \
--is-int-calc-required false
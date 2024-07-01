INPUT=$"test-bed/input.txt"
OUTPUT_PRINCIPAL=$"test-bed/cf_loan_finacle_output_performing"
LOG_FILE=$"test-bed/cf_loan_finacle_log_performing.txt"
DIAGNOSTICS_FILE=$"test-bed/cf_loan_finacle_diag-log_performing.txt"

cargo run -- \
--as-on-date 31-05-2022 \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--input-file ${INPUT} \
--log-file ${LOG_FILE} \
--output-file-principal ${OUTPUT_PRINCIPAL} \
--log-level debug \
--diagnostics-flag false

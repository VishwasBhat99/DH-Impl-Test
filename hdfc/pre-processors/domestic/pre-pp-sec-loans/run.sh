INPUT=$"test-bed/final-input-1.xlsx"
YILED=$"test-bed/yield.csv"
OUTPUT=$"test-bed/output.txt"
NON_CONC=$"test-bed/non_concat.txt"
LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diag-log.txt"

cargo run --release -- \
--master-file ${INPUT} \
--yield-file ${YILED} \
--non-concat-file-path ${NON_CONC} \
--output-file ${OUTPUT} \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--sec-cashflows-sheet Sheet1 \
--sec-master-sheet Sheet2 \
--log-level trace \
--diagnostics-flag true \
--as-on-date 27-01-2019

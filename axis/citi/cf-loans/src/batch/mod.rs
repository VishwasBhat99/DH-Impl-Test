use rbconcurrency::WorkerPool;
use slog::Logger;
use std::io::BufRead;
use std::sync::mpsc;
use std::time::SystemTime;

mod batch_params;
pub mod batch_process_report;
mod batch_processor_closure;
mod converter;
pub mod duration_extensions;
mod index;
mod io_helpers;
mod output_descriptor;
mod result_descriptor_adder_thread;
mod results_descriptor;
mod writer_thread;

use self::batch_params::BatchParams;
use self::batch_process_report::BatchProcessReport;
use self::result_descriptor_adder_thread::result_descriptor_adder_thread;
use super::macros;
use rbdate::NaiveDate;
use sdb_day_convention::conventions::Conventions;

pub fn process(
    batch_size: u32,
    thread_count: u8,
    input_file_path: &str,
    output_file_path: &str,
    log: &Logger,
    diag_log: &Logger,
    as_on_date: NaiveDate,
    is_contractual: bool,
) -> BatchProcessReport {
    let input_file_reader = io_helpers::buf_reader(input_file_path);
    let mut line_read_iterator = input_file_reader.lines();
    let mut lines_count = 0;

    let (writer_sender, writer_receiver) = mpsc::channel();
    let output_writer_thread = writer_thread::new_writer_thread(
        "Output Writer".to_string(),
        diag_log.clone(),
        writer_receiver,
        output_file_path.to_string(),
    );

    let (cashflow_details_sender, cashflow_details_receiver) = mpsc::channel();
    let (total_cashflow_sender, total_cashflow_receiver) = mpsc::channel();

    let cashflow_details_accumulator_thread =
        result_descriptor_adder_thread(cashflow_details_receiver, total_cashflow_sender);

    let batch_processor_closure = batch_processor_closure::batch_processor_closure();

    let thread_count_as_usize = thread_count as usize;
    let batch_size_as_usize = batch_size as usize;
    let worker_pool = WorkerPool::new(thread_count, batch_processor_closure);
    let line_batch_size = (batch_size_as_usize) / thread_count_as_usize;

    let mut batched_lines_sender_pair = Vec::with_capacity(thread_count_as_usize);
    let mut batched_lines = Vec::with_capacity(batch_size_as_usize);

    let start_time = SystemTime::now();

    let mut is_eof = false;

    line_read_iterator.next();
    loop {
        let line_opt = log_measurements!(
            diag_log,
            [format!("Type: ReadLine, Identifier: {}", lines_count + 1)],
            line_read_iterator.next()
        );

        if let Some(line) = line_opt {
            lines_count += 1;
            if let Ok(underlying_string) = line {
                batched_lines.push(underlying_string);
            } else {
                // This line contains an erroneous string.
                log_error!(
                    log,
                    "Invalid string encountered in line. Line No: {}. Value: {:?}",
                    lines_count,
                    line
                );
            }
        } else {
            is_eof = true;
        }

        if (batched_lines.len() == line_batch_size) || (is_eof) {
            batched_lines_sender_pair.push(BatchParams::new(
                batched_lines,
                writer_sender.clone(),
                cashflow_details_sender.clone(),
                log.clone(),
                diag_log.clone(),
                as_on_date,
                is_contractual,
            ));

            batched_lines = Vec::with_capacity(batch_size_as_usize);

            if (batched_lines_sender_pair.len() == thread_count_as_usize) || (is_eof) {
                let results = worker_pool.submit(batched_lines_sender_pair);
                for _result in results { /* This blocks.  */ }

                batched_lines_sender_pair = Vec::with_capacity(thread_count_as_usize);
            }
        }

        if is_eof {
            break;
        }
    }

    // TODO: Renaming required.
    drop(writer_sender);
    output_writer_thread.join().unwrap();

    drop(cashflow_details_sender);
    let cf_result_descriptors_added = total_cashflow_receiver.recv().unwrap();
    cashflow_details_accumulator_thread.join().unwrap();

    let end_time = SystemTime::now();
    let duration = end_time.duration_since(start_time).unwrap();

    BatchProcessReport {
        time_taken_seconds: duration.as_secs(),
        thread_count,
        batch_size,
        inputs_processed: cf_result_descriptors_added.inputs_count,
        total_cashflows: cf_result_descriptors_added.cashflows_count,
        total_successful_records: cf_result_descriptors_added.successful_outputs_count,
        total_faulty_records: cf_result_descriptors_added.erroneous_outputs_count,
        total_amount_input: cf_result_descriptors_added.total_amount_input,
        total_principal_output: cf_result_descriptors_added.total_principal_output,
        total_interest_output: cf_result_descriptors_added.total_interest_output,
    }
}

#[allow(dead_code)]
impl converter::account::Account {
    fn identifier(&self) -> String {
        self.get_account_number().to_string()
    }
}

use super::structs::LLGAggregatedRecord;
use aggregator::cashflow_organizer::cashflow_aggregation::CashflowAggregated;
use aggregator::llg::llg_key::LLGKey;
use aggregator::reports::llgs_report::OutstandingAmountReport;
use aggregator::writer::file_writer_thread_report::AggregateWriterThreadReport;
use macros;
use sdb_io;
use slog::Logger;
use std::collections::HashMap;
use std::io::Write;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::Builder;
use std::thread::JoinHandle;
use std::time::SystemTime;

pub fn new_writer_thread(
    thread_no: u32,
    file_path: String,
    aggregate_recv: Receiver<LLGAggregatedRecord>,
    diag_logger: Logger,
    closing_report_sender: Sender<AggregateWriterThreadReport>,
) -> JoinHandle<()> {
    let mut thread_builder = Builder::new();
    thread_builder = thread_builder.name(format!("File Writer Thread No: {}", thread_no));

    thread_builder
        .spawn(move || {
            let full_path = format!("{}.txt", file_path);
            let mut wrtr = match sdb_io::buf_file_wrtr(&full_path, None) {
                Ok(w) => w,
                Err(e) => panic!(format!(
                    "Cannot write to file at path: '{}'. Error: {}",
                    full_path, e
                )),
            };

            let mut reusable_aggregates_string_o_a = String::new();
            let mut reusable_aggregates_string_ir = String::new();
            let empty_aggregates_value = CashflowAggregated::empty_value_string();;

            let mut total_records_written = 0;
            let mut amount_totals_per_llg_report: HashMap<LLGKey, OutstandingAmountReport> =
                HashMap::new();

            loop {
                match aggregate_recv.recv() {
                    Ok(aggregates_record) => {
                        let report = log_measurements!(
                            diag_logger,
                            [format!(
                                "Type: AggregateToString, Identifier: {:?}",
                                aggregates_record.prefix
                            )],
                            {
                                append_to_string(
                                    &mut reusable_aggregates_string_o_a,
                                    &aggregates_record,
                                    &empty_aggregates_value,
                                )
                            }
                        );
                        build_report(&mut amount_totals_per_llg_report, report);

                        log_measurements!(
                            diag_logger,
                            [format!(
                                "Type: WriteAggregate, Identifier: {:?}",
                                aggregates_record.prefix
                            )],
                            {
                                wrtr.write(&reusable_aggregates_string_o_a.as_bytes())
                                    .expect("Could not write to file");
                                wrtr.flush().unwrap();
                            }
                        );

                        reusable_aggregates_string_o_a.clear();
                        reusable_aggregates_string_ir.clear();
                        total_records_written += 1;
                    }
                    Err(_error) => {
                        // The sender has disconnected.
                        log_measurements!(
                            diag_logger,
                            [format!("Type: FlushWriter, Identifier: Writer")],
                            wrtr.flush()
                                .expect("Unable to flush record writer contents")
                        );
                        closing_report_sender
                            .send(AggregateWriterThreadReport {
                                thread_no,
                                total_records_written,
                                llg_amounts_report: amount_totals_per_llg_report,
                            })
                            .expect(&format!(
                                "Thread No {} couldn't send its report to the main thread",
                                thread_no
                            ));
                        break;
                    }
                }
            }
        })
        .expect("Could not create writer thread")
}

fn append_to_string(
    string_o_a: &mut String,
    aggregates_record: &LLGAggregatedRecord,
    empty_string: &str,
) -> (LLGKey, OutstandingAmountReport) {
    let llg = aggregates_record.prefix.subtype_id.clone();
    let mut o_a_amount_report = OutstandingAmountReport::new();

    let c_o_a_string = aggregates_record.prefix.to_string();
    string_o_a.push_str(&c_o_a_string);

    for aggregate in aggregates_record.aggregates.iter() {
        match aggregate {
            None => {
                string_o_a.push_str(empty_string);
            }
            Some(a) => {
                o_a_amount_report.outstanding_amount += a.o_a.amount;
                string_o_a.push_str(&a.o_a.to_string());
            }
        }
    }
    string_o_a.push_str("\n");
    (llg, o_a_amount_report)
}

fn build_report(
    report: &mut HashMap<LLGKey, OutstandingAmountReport>,
    new: (LLGKey, OutstandingAmountReport),
) {
    // Bug: This is such a weird limitation with Rust? Why can't I just have a mutable borrow and
    // modify the value in place? Why do I need `clone`? This is bad and unnecessary.
    // Post of the Rust users group when you have time.
    let mut existing = report
        .entry(new.0.clone())
        .or_insert(OutstandingAmountReport::new())
        .clone();
    existing += new.1;
    report.insert(new.0, existing);
}

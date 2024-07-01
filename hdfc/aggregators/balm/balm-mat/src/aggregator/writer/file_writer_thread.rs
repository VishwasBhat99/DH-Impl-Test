use super::structs::CashflowAggregatedType;
use super::structs::LLGAggregatedRecord;
use aggregator::llg_key::LLGKey;
use aggregator::reports::llgs_report::PrincipalInterestAmountReport;
use aggregator::structs::CashflowAggregated;
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
            let full_path = format!("{}-{}.txt", file_path, thread_no);
            let mut wrtr = match sdb_io::buf_file_wrtr(&full_path, None) {
                Ok(w) => w,
                Err(e) => panic!(format!(
                    "Cannot write to file at path: '{}'. Error: {}",
                    full_path, e
                )),
            };

            let mut reusable_aggregates_string_int = String::new();
            let mut reusable_aggregates_string_slr = String::new();
            let mut reusable_aggregates_string_irs = String::new();
            let empty_aggregates_value = CashflowAggregated::empty_value_string();

            let mut total_records_written = 0;
            let mut amount_totals_per_llg_report: HashMap<LLGKey, PrincipalInterestAmountReport> =
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
                                    &mut reusable_aggregates_string_int,
                                    &mut reusable_aggregates_string_slr,
                                    &mut reusable_aggregates_string_irs,
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
                                wrtr.write(&reusable_aggregates_string_int.as_bytes())
                                    .expect("Could not write to file");
                                wrtr.write(&reusable_aggregates_string_slr.as_bytes())
                                    .expect("Could not write to file");
                                wrtr.write(&reusable_aggregates_string_irs.as_bytes())
                                    .expect("Could not write to file");
                                wrtr.flush().expect("Unable to flush writer.");
                            }
                        );

                        reusable_aggregates_string_irs.clear();
                        reusable_aggregates_string_int.clear();
                        reusable_aggregates_string_slr.clear();
                        total_records_written += 3;
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
    string_int: &mut String,
    string_slr: &mut String,
    string_irs: &mut String,
    aggregates_record: &LLGAggregatedRecord,
    empty_string: &str,
) -> (LLGKey, PrincipalInterestAmountReport) {
    let llg = aggregates_record.prefix.subtype_id.clone();
    let mut p_i_amount_report = PrincipalInterestAmountReport::new();

    let c_int_string = aggregates_record
        .prefix
        .to_string(CashflowAggregatedType::Int);
    let c_slr_string = aggregates_record
        .prefix
        .to_string(CashflowAggregatedType::Slr);
    let c_irs_string = aggregates_record
        .prefix
        .to_string(CashflowAggregatedType::Irs);

    string_int.push_str(&c_int_string);
    string_slr.push_str(&c_slr_string);
    string_irs.push_str(&c_irs_string);

    for aggregate in aggregates_record.aggregates.iter() {
        match aggregate {
            None => {
                string_int.push_str(empty_string);
                string_slr.push_str(empty_string);
                string_irs.push_str(empty_string);
            }
            Some(a) => {
                p_i_amount_report.principal_amount += a.slr.amount;
                p_i_amount_report.interest_amount += a.int.amount;
                string_int.push_str(&a.int.to_string());
                string_slr.push_str(&a.slr.to_string());
                string_irs.push_str(&a.irs.to_string());
            }
        }
    }
    // As "|" is not required at last
    string_int.pop();
    string_slr.pop();
    string_irs.pop();

    string_int.push_str("\n");
    string_slr.push_str("\n");
    string_irs.push_str("\n");

    (llg, p_i_amount_report)
}

fn build_report(
    report: &mut HashMap<LLGKey, PrincipalInterestAmountReport>,
    new: (LLGKey, PrincipalInterestAmountReport),
) {
    // Bug: This is such a weird limitation with Rust? Why can't I just have a mutable borrow and
    // modify the value in place? Why do I need `clone`? This is bad and unnecessary.
    // Post of the Rust users group when you have time.
    let mut existing = report
        .entry(new.0.clone())
        .or_insert(PrincipalInterestAmountReport::new())
        .clone();
    existing += new.1;
    report.insert(new.0, existing);
}

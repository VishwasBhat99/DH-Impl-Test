mod file_writer_thread;
pub mod file_writer_thread_report;
mod structs;
pub mod summaries;
pub mod writer_handle;
use self::summaries::LLGSummaryRecord;
use aggregator::cashflow_organizer::cashflow_aggregation::LLGAggregateOnDay;
use aggregator::llg::llg_key::LLGKey;
use aggregator::reports::llg_summaries_report::LLGSummariesReport;
use aggregator::writer::summaries::LLGSummaryRecordGrouped;
use aggregator::writer::writer_handle::AggregateWriterReport;
use aggregator::writer::writer_handle::WriterHandles;
use rbdate::NaiveDate;
use sdb_io;
use slog::Logger;
use std::io::Write;

pub struct WriterConstants {
    as_on: String,
}

pub struct Writer {
    writer_handles: writer_handle::WriterHandles,
    successive_split_indices: [usize; 1],
    output_path: String,
    constant: WriterConstants,
    summaries: Vec<LLGSummaryRecordGrouped>,
}

impl Writer {
    pub fn new(output_path: &str, as_on_date: &NaiveDate, diag_log: Logger) -> Writer {
        Writer {
            writer_handles: WriterHandles::new(output_path, diag_log),
            successive_split_indices: [28],
            output_path: output_path.to_string(),
            constant: WriterConstants {
                as_on: as_on_date.format("%d-%m-%Y").to_string(),
            },
            summaries: Vec::new(),
        }
    }
}

impl Writer {
    pub fn flush(&mut self, llg: LLGKey, mut aggregated_cfs_vec: Vec<Option<LLGAggregateOnDay>>) {
        // Store summaries within the Writer.
        let grouped_summary = summaries::summary_for_llg(llg.clone(), &aggregated_cfs_vec);
        self.summaries.push(grouped_summary);

        // Split aggregates and send to writers.
        for (i, split_point) in self.successive_split_indices.iter().enumerate() {
            let remainder_aggregates = aggregated_cfs_vec.split_off(split_point.clone());
            let prefix = structs::LLGRecordPrefix {
                subtype_id: llg.clone(),
                as_on: self.constant.as_on.to_string(),
            };
            let aggregated_record = structs::LLGAggregatedRecord {
                prefix,
                aggregates: aggregated_cfs_vec,
            };
            self.writer_handles.send_to_writer(i, aggregated_record);
            aggregated_cfs_vec = remainder_aggregates;
        }
    }
}

impl Writer {
    pub fn close(self) -> (LLGSummariesReport, AggregateWriterReport) {
        let summary_report = self.flush_summary();
        let aggregate_writer_report = self.writer_handles.end_writer();

        (summary_report, aggregate_writer_report)
    }

    fn flush_summary(&self) -> LLGSummariesReport {
        let mut summary_string = String::new();
        let mut llg_summaries_report = LLGSummariesReport::new();
        for grouped_summary in &self.summaries {
            llg_summaries_report.build_with(grouped_summary);
            summary_string.push_str(&self.grouped_summary_record(grouped_summary));
        }
        let full_path = format!("{}-summary.txt", &self.output_path);
        let mut wrtr = match sdb_io::buf_file_wrtr(&full_path, None) {
            Ok(w) => w,
            Err(e) => panic!(format!(
                "Cannot write to file at path: '{}'. Error: {}",
                full_path, e
            )),
        };

        wrtr.write(summary_string.as_bytes()).expect(&format!(
            "Couldn't write to summary file. String: {}",
            summary_string
        ));
        wrtr.flush()
            .expect("Unable to flush summary writer contents");

        llg_summaries_report
    }

    fn grouped_summary_record(&self, grouped_summary: &LLGSummaryRecordGrouped) -> String {
        format!(
            "{}\n",
            self.summary_record_string(&grouped_summary.subtype_id, &grouped_summary.o_a),
        )
    }

    fn summary_record_string(&self, subtype_id: &LLGKey, summary: &LLGSummaryRecord) -> String {
        format!(
            "{}|{}|{}|{}",
            self.constant.as_on, subtype_id, summary.amount, summary.rate
        )
    }
}

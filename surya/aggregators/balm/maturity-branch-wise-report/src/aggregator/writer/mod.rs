mod file_writer_thread;
pub mod file_writer_thread_report;
mod structs;
pub mod summaries;
pub mod writer_handle;
use self::summaries::LLGSummaryRecord;
use aggregator::llg_key::LLGKey;
use aggregator::reports::llg_summaries_report::LLGSummariesReport;
use aggregator::structs::LLGAggregateOnDay;
use aggregator::writer::summaries::LLGSummaryRecordGrouped;
use aggregator::writer::writer_handle::AggregateWriterReport;
use aggregator::writer::writer_handle::WriterHandles;
use rbdate::NaiveDate;
use sdb_io;
use slog::Logger;
use std::io::Write;

pub struct WriterConstants {
    scheme_id: String,
    as_on: String,
}

pub struct Writer {
    writer_handles: writer_handle::WriterHandles,
    successive_split_indices: [usize; 6],
    output_path: String,
    constant: WriterConstants,
    summaries: Vec<LLGSummaryRecordGrouped>,
}

impl Writer {
    pub fn new(output_path: &str, as_on_date: &NaiveDate, diag_log: Logger) -> Writer {
        Writer {
            writer_handles: WriterHandles::new(output_path, diag_log),
            // These indicies are known from the SQL DBs being used at the time of implementation.
            // Each number represents the count of elements between one number and the next.
            // Specifically, the numbers in the DB are:
            //
            // 1-165, 166-330, 331-366, 367-531, 532-696, 697-717
            successive_split_indices: [165, 165, 36, 165, 165, 21],
            output_path: output_path.to_string(),
            constant: WriterConstants {
                // Hard-code `Master`
                scheme_id: "Master".to_string(),
                as_on: format!("{}", as_on_date.format("%d-%m-%Y")),
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
                item_id: llg.item_id.to_string(),
                dim_id: llg.dim_id.to_string(),
                as_on: self.constant.as_on.to_string(),
                currency_id: llg.currency.clone(),
            };
            let aggregated_record = structs::LLGAggregatedRecord {
                prefix,
                aggregates: aggregated_cfs_vec,
            };
            if llg.cf_type == "O" {
                self.writer_handles.send_to_writer(i + 6, aggregated_record);
            } else {
                self.writer_handles.send_to_writer(i, aggregated_record);
            }
            aggregated_cfs_vec = remainder_aggregates;
        }
    }
    pub fn flush_overdue(
        &mut self,
        llg: LLGKey,
        aggregated_cfs_vec: Vec<Option<LLGAggregateOnDay>>,
    ) {
        // Store summaries for overdue
        let grouped_summary = summaries::summary_for_llg(llg.clone(), &aggregated_cfs_vec);
        self.summaries.push(grouped_summary)
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
            summary_string.push_str(&self.grouped_summary_record(grouped_summary))
        }

        let full_path = format!("{}-summary.txt", &self.output_path);
        let mut wrtr = match sdb_io::buf_file_wrtr(&full_path, None) {
            Ok(w) => w,
            Err(e) => panic!(
                "Cannot write to file at path: '{}'. Error: {}",
                full_path, e
            ),
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
            "{}\n{}\n{}\n",
            self.summary_record_string(
                &grouped_summary.currency,
                &grouped_summary.subtype_id,
                &grouped_summary.int
            ),
            self.summary_record_string(
                &grouped_summary.currency,
                &grouped_summary.subtype_id,
                &grouped_summary.irs
            ),
            self.summary_record_string(
                &grouped_summary.currency,
                &grouped_summary.subtype_id,
                &grouped_summary.slr
            )
        )
    }

    fn summary_record_string(
        &self,
        currency: &str,
        subtype_id: &LLGKey,
        summary: &LLGSummaryRecord,
    ) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            subtype_id.category,
            subtype_id.item_id,
            subtype_id.dim_id,
            self.constant.as_on,
            currency,
            summary.ttype.to_string(),
            subtype_id.cf_type,
            summary.amount,
            summary.rate
        )
    }
}

use aggregator::writer::file_writer_thread::new_writer_thread;
use aggregator::writer::file_writer_thread_report::AggregateWriterThreadReport;
use aggregator::writer::structs::LLGAggregatedRecord;
use slog::Logger;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

pub struct WriterHandles {
    handles: Vec<WriterHandle>,
}

pub type AggregateWriterReport = Vec<AggregateWriterThreadReport>;

impl WriterHandles {
    pub fn new(output_path: &str, diag_log: Logger) -> WriterHandles {
        //Todo : check if we can avoid having this vec as we have only one element in it
        let mut handles = Vec::with_capacity(1);
        let handle = writer_handles(0, output_path, diag_log.clone());
        handles.push(handle);

        WriterHandles { handles }
    }

    pub fn send_to_writer(&self, writer_pos: usize, record: LLGAggregatedRecord) {
        self.handles[writer_pos]
            .aggregate_sender
            .send(record)
            .expect(&format!(
                "Unable to send record to writer. Writer number: {}",
                writer_pos
            ))
    }

    /// Flushes all writers' content to disk, and kills the threads that contain them.
    pub fn end_writer(self) -> AggregateWriterReport {
        let mut writer_thread_reports = Vec::with_capacity(1);
        for handle in self.handles {
            drop(handle.aggregate_sender);
            let report = handle
                .report_receiver
                .recv()
                .expect("Could not recieve report from writer thread.");
            writer_thread_reports.insert(report.thread_no as usize, report);
            handle.thread_handle.join().unwrap();
        }

        writer_thread_reports
    }
}

pub struct WriterHandle {
    pub aggregate_sender: Sender<LLGAggregatedRecord>,
    pub thread_handle: JoinHandle<()>,
    pub report_receiver: Receiver<AggregateWriterThreadReport>,
}

pub fn writer_handles(thread_no: u32, file_path: &str, diag_logger: Logger) -> WriterHandle {
    let (sender, receiver) = mpsc::channel();
    let (report_sender, report_receiver) = mpsc::channel();
    let thread_handle = new_writer_thread(
        thread_no,
        file_path.to_string(),
        receiver,
        diag_logger,
        report_sender,
    );

    WriterHandle {
        aggregate_sender: sender,
        thread_handle: thread_handle,
        report_receiver,
    }
}

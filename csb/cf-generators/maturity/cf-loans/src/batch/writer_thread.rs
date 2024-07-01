use super::index;
use super::io_helpers;
use macros;
use protobuf::Message;
use slog::Logger;
use std::io::Write;
use std::sync::mpsc::Receiver;
use std::thread::Builder;
use std::thread::JoinHandle;
use std::time::SystemTime;

/// The `name`, and `diag_logger` parameters are used to log the duration it takes to write data to the file. Logs are only generated in the debug builds.
pub fn new_writer_thread(
    name: String,
    diag_logger: Logger,
    receiver_channel: Receiver<Vec<(String, Vec<u8>)>>,
    file_path: String,
) -> JoinHandle<()> {
    let mut idx_writer = io_helpers::buf_file_writer(&file_path, ".idx", None);
    let mut cf_writer = io_helpers::buf_file_writer(&file_path, ".cf", None);

    let mut thread_builder = Builder::new();
    thread_builder = thread_builder.name(name.clone());

    thread_builder.spawn(move|| {

        let mut indices = index::Index::new();
        let mut end: i64 = 0;

        loop {
            match receiver_channel.recv() {
                Ok(outputs) => {
                    for (account_no, output_bytes) in outputs {

                        log_measurements!(
                            diag_logger,
                            [format!("Type: WriteOutput, Identifier: {}", account_no)],
                            {
                                match cf_writer.write(&output_bytes) {
                                    Ok(_) => { },
                                    Err(e) => {
                                        panic!("Program failed when writing a record in writer: '{}'. Error: {}. Aborting execution.", name, e);
                                    }
                                };
                            }
                        );

                        log_measurements!(
                            diag_logger,
                            [format!("Type: WriteOutputIndex, Identifier: {}", account_no)],
                            {
                                end += output_bytes.len() as i64;

                                indices.set_key(account_no);
                                indices.set_offset(end);
                                match indices.write_length_delimited_to_writer(&mut idx_writer) {
                                    Ok(_) => { },
                                    Err(e) => {
                                        panic!("Program failed when writing the index for writer thread: '{}' Error: {}. Aborting execution.", name, e);
                                    }
                                }
                            }
                        );

                    }
                },
                Err(_error) => {
                    // The sender has disconnected.
                    log_measurements!(
                        diag_logger,
                        [format!("Type: FlushWriter, Identifier: CashflowWriter")],
                        io_helpers::flush_contents(cf_writer, "Cashflow Writer")
                    );
                    log_measurements!(
                        diag_logger,
                        [format!("Type: FlushWriter, Identifier: IndexWriter")],
                        io_helpers::flush_contents(idx_writer, "Index Writer")
                    );
                    break;
                }
            }
        }
    }).unwrap()
}

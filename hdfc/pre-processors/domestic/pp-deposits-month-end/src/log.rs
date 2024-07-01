use rbdate;
use sdb_io::buf_file_wrtr;
use slog;
use slog::Drain;
use slog_async;
use slog_async::OverflowStrategy;
use slog_term;
use std::io::Result;
use std::io::Write;

pub fn setup_loggers(
    log_file_path: &str,
    diagnostics_file_path: &str,
) -> (slog::Logger, slog::Logger) {
    let log_file = buf_file_wrtr(log_file_path, None).expect("Error while creating log file.");
    let decorator = slog_term::PlainDecorator::new(log_file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain_builder = slog_async::Async::new(drain);
    let logger_drain = drain_builder
        .overflow_strategy(OverflowStrategy::Block)
        .build()
        .fuse();
    let logger = slog::Logger::root(logger_drain, o!());

    let diag_log_file = buf_file_wrtr(diagnostics_file_path, None)
        .expect("Error while creating diagnostics log file.");

    let diag_decorator = slog_term::PlainDecorator::new(diag_log_file);
    let diag_drain_builder = slog_term::FullFormat::new(diag_decorator);
    let diag_drain = diag_drain_builder
        .use_custom_timestamp(local_timestamp_utc)
        .build()
        .fuse();

    let async_diag_drain_builder = slog_async::Async::new(diag_drain);
    let async_diag_drain = async_diag_drain_builder
        .overflow_strategy(OverflowStrategy::Block)
        .build()
        .fuse();
    let diagnostics_logger = slog::Logger::root(async_diag_drain, o!());

    (logger, diagnostics_logger)
}

fn local_timestamp_utc(io: &mut Write) -> Result<()> {
    write!(io, "{:?}", rbdate::current_time_utc())
}

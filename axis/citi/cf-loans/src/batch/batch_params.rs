use super::results_descriptor::ResultsDescriptor;
use rbdate::NaiveDate;
use sdb_day_convention::conventions::Conventions;
use slog::Logger;
use std::sync::mpsc::Sender;

/// BatchParams is a structure used to pass the fields required to precess input lines in an isolated thread.
pub struct BatchParams {
    pub lines: Vec<String>,
    pub outputs_sender: Sender<Vec<(String, Vec<u8>)>>,
    pub result_descriptor_sender: Sender<ResultsDescriptor>,
    pub as_on_date: NaiveDate,
    pub is_contractual: bool,
    pub logger: Logger,
    pub diagnostics_logger: Logger,
}

impl BatchParams {
    pub fn new(
        lines: Vec<String>,
        outputs_sender: Sender<Vec<(String, Vec<u8>)>>,
        result_descriptor_sender: Sender<ResultsDescriptor>,
        logger: Logger,
        diagnostics_logger: Logger,
        as_on_date: NaiveDate,
        is_contractual: bool,
    ) -> BatchParams {
        BatchParams {
            lines,
            outputs_sender,
            result_descriptor_sender,
            as_on_date,
            is_contractual,
            logger,
            diagnostics_logger,
        }
    }
}

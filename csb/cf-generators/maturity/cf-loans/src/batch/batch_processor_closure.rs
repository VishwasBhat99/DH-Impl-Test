use super::converter;
use super::macros;
use super::output_descriptor::AccountDescriptor;
use super::BatchParams;
use batch::converter::input::InputParseResult::*;
use batch::results_descriptor::ResultsDescriptor;
use protobuf::Message;
use rbdate::DateParser;
use std::time::SystemTime;

#[inline(always)]
pub fn batch_processor_closure() -> fn(BatchParams) -> () {
    move |batch_params: BatchParams| {
        let dmy = DateParser::new("%d-%m-%Y".to_string(), true);

        let mut output_descriptor_total = AccountDescriptor::new();

        let mut inputs_count = 0;
        let mut successful_outputs_count = 0;
        let mut erroneous_outputs_count = 0;

        let mut outputs = Vec::new();

        for l in batch_params.lines {
            inputs_count += 1;
            let input_result = log_measurements!(
                batch_params.diagnostics_logger,
                [format!(
                    "Type: DeserialiseInput, Identifier: {}",
                    inputs_count
                )],
                converter::input::Input::new_from_line(&l, &dmy)
            );
            match input_result {
                Some(input) => {
                    let identifier = input.identifier();

                    let res = log_measurements!(
                        batch_params.diagnostics_logger,
                        [format!("Type: InputToOutput, Identifier: {}", identifier)],
                        converter::convert(
                            input,
                            batch_params.convention,
                            batch_params.as_on_date,
                            batch_params.is_contractual,
                            &batch_params.logger
                        )
                    );

                    match res {
                        Ok((output, op_descriptor)) => {
                            successful_outputs_count += 1;
                            output_descriptor_total += op_descriptor;

                            let output_serialised = log_measurements!(
                                batch_params.diagnostics_logger,
                                [format!("Type: SerialiseOutput, Identifier: {}", identifier)],
                                output.write_length_delimited_to_bytes().unwrap()
                            );

                            outputs.push((output.acc_no, output_serialised));
                        }
                        Err(error) => {
                            erroneous_outputs_count += 1;
                            log_warn!(batch_params.logger, "{}", error)
                        }
                    }
                }
                Error(reason) => {
                    log_error!(
                        batch_params.logger,
                        "Could not parse line {} due to error: {}",
                        &l,
                        reason
                    );
                }
            }
        }

        // TODO: Make an init that takes in values plus `OutputDescriptor`
        let results_descriptor = ResultsDescriptor {
            inputs_count,
            cashflows_count: output_descriptor_total.cashflows_count,
            successful_outputs_count,
            erroneous_outputs_count,
            total_amount_input: output_descriptor_total.total_amount_input,
            total_principal_output: output_descriptor_total.total_principal_output,
            total_interest_output: output_descriptor_total.total_interest_output,
        };
        batch_params
            .result_descriptor_sender
            .send(results_descriptor)
            .unwrap();
        batch_params.outputs_sender.send(outputs).unwrap();
    }
}

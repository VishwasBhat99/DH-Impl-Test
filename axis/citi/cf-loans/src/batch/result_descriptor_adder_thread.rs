use super::results_descriptor::ResultsDescriptor;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::Builder;
use std::thread::JoinHandle;

pub fn result_descriptor_adder_thread(
    result_descriptor_receiver: Receiver<ResultsDescriptor>,
    added_result_descriptor_sender: Sender<ResultsDescriptor>,
) -> JoinHandle<()> {
    let thread_name = "Cashflow Accumulator Thread".to_string();
    let mut thread_builder = Builder::new();
    thread_builder = thread_builder.name(thread_name);

    thread_builder
        .spawn(move || {
            let mut total_cf_descriptor = ResultsDescriptor::new();
            loop {
                match result_descriptor_receiver.recv() {
                    Ok(cf_descriptor) => {
                        total_cf_descriptor += cf_descriptor;
                    }
                    Err(_e) => {
                        added_result_descriptor_sender
                            .send(total_cf_descriptor)
                            .unwrap();
                        break;
                    }
                }
            }
        })
        .unwrap()
}

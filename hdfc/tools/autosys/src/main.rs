extern crate serde;
use clap::{App, Arg};
use run::run;
use status_checker::batch_status_checker;
extern crate clap;
extern crate rand;

mod run;
mod status_checker;

fn main() {
    let matches = App::new("autosys")
        .version("1.1.4225")
        .about("Program to automate multiple runcontrol streams")
        .arg(
            Arg::with_name("as-on-date")
                .long("as-on-date")
                .required(true)
                .takes_value(true)
                .help("AsOnDate"),
        )
        .arg(
            Arg::with_name("batch-id")
                .long("batch-id")
                .required(true)
                .takes_value(true)
                .help("BatchId"),
        )
        .arg(
            Arg::with_name("stream-ids")
                .long("stream-ids")
                .required(true)
                .multiple(true)
                .value_delimiter(",")
                .takes_value(true)
                .help("StreamIds"),
        )
        .arg(
            Arg::with_name("connection-string")
                .long("connection-string")
                .required(true)
                .takes_value(true)
                .help("API Connection String"),
        )
        .arg(
            Arg::with_name("max-retry")
                .long("max-retry")
                .required(true)
                .takes_value(true)
                .help("Maximum retry times until stream success"),
        )
        .arg(
            Arg::with_name("wait-time-in-sec")
                .long("wait-time-in-sec")
                .required(true)
                .takes_value(true)
                .help("Wait Time in seconds to check status again"),
        )
        .arg(
            Arg::with_name("accept-invalid-certs")
                .long("accept-invalid-certs")
                .required(false)
                .takes_value(true)
                .default_value("true")
                .help("Validates the SSL/TLS certificates"),
        )
        .get_matches();

    let as_on_date = matches
        .value_of("as-on-date")
        .expect("Error getting `AsOnDate`.")
        .to_string();
    let batch_id = matches
        .value_of("batch-id")
        .expect("Error getting `BatchId`.")
        .parse::<i64>()
        .expect("Cannot parse batch id arg as integer");
    let stream_id: Vec<i64> = matches
        .values_of("stream-ids")
        .expect("Error getting `StreamIds`.")
        .map(|n| n.parse().expect("Error parsing `StreamIds`."))
        .collect();
    let connection_string = matches
        .value_of("connection-string")
        .expect("Error getting `RC Connection String`.")
        .to_string();
    let max_retry = matches
        .value_of("max-retry")
        .expect("Error getting `max-retry`.")
        .parse::<i64>()
        .expect("Cannot parse max retry arg as integer");
    let wait_time = matches
        .value_of("wait-time-in-sec")
        .expect("Error getting `wait-time-in-sec`.")
        .parse::<u64>()
        .expect("Cannot parse wait time arg as integer");
    let accept_invalid_certs= matches 
    .value_of("accept-invalid-certs")
    .expect("Error getting accept invalid certs flag")
    .parse::<bool>()
    .expect("Cannot parse accept invalid certs arg as bool");
    run(&as_on_date, batch_id, &stream_id, &connection_string,&accept_invalid_certs);
    let final_run_status = batch_status_checker(
        &as_on_date,
        batch_id,
        &stream_id,
        &connection_string,
        wait_time,
        max_retry,
        &accept_invalid_certs
    );
    if final_run_status {
        println!("Batch run success completed.");
        std::process::exit(0)
    } else {
        println!("Batch run Failed.");
        std::process::exit(1)
    }
}

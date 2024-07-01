use serde::{Deserialize, Serialize};
use std::thread::sleep;
use std::time::Duration;

#[derive(Serialize, Debug)]
struct ReqBody {
    as_on_date: String,
    batch_id: i64,
}

#[derive(Deserialize, Debug)]
struct FlowFullStatus {
    flow_id: i64,
    flow_name: String,
    flow_start_time: String,
    flow_end_time: String,
    flow_status: String,
    tt_by_flow_secs: i64,
    flow_error_msg: String,
    process: Vec<ProcessFullStatus>,
}

#[derive(Deserialize, Debug)]
struct ProcessFullStatus {
    process_id: i64,
    process_name: String,
    process_start_time: String,
    process_end_time: String,
    process_status: String,
    process_error_msg: String,
    tt_by_process_secs: i64,
    total_num_recs: i64,
    success_process_recs: i64,
    failed_process_recs: i64,
    principal_input_amt: f64,
    principal_output_amt: f64,
    interest_output_amt: f64,
}

#[derive(Deserialize, Debug)]
struct BatchFullStatus {
    batch_id: i64,
    as_on_date: String,
    streams: Vec<StreamFullStatus>,
}

#[derive(Deserialize, Debug)]
struct StreamFullStatus {
    stream_id: i64,
    stream_name: String,
    stream_start_time: String,
    stream_end_time: String,
    stream_status: String,
    tt_by_stream_secs: i64,
    stream_error_msg: String,
    flows: Vec<FlowFullStatus>,
}

pub fn batch_status_checker(
    as_on_date: &String,
    batch_id: i64,
    stream_id: &Vec<i64>,
    connection_string: &String,
    wait_time: u64,
    max_retry: i64,
    accept_invalid_certs:&bool
) -> bool {
    let mut retry_count = 0;
    let mut is_fail = true;
    let mut is_success = false;
    let req_body = ReqBody {
        as_on_date: as_on_date.to_string(),
        batch_id: batch_id,
    };
    let url = format!("{}/batch_full_status/1", connection_string);
    let client_builder = reqwest::ClientBuilder::new().danger_accept_invalid_certs(*accept_invalid_certs).timeout(Duration::from_secs(120));
    let client = client_builder
        .build()
        .expect("Could not create an api client!!");
    loop {
        let mut batch_run_completed=true;
        is_fail = false;
        is_success = false;
        let resp = client.post(&url).json(&req_body).send();
        match &resp {
            Ok(_val) => {}
            Err(err) => {
                println!(
                    "Fetching Run Status again in 15 sec!!: {}\n URL: {}\nBODY: {:#?}\nERROR:{:#?}",
                    err, url, req_body, resp
                );
                sleep(Duration::from_secs(15));
                continue;
            }
        }
        let resp_body: BatchFullStatus = resp
            .expect("Unexpected error on batch full status response unwrap")
            .json()
            .expect("Cannot read response.");
        for streams in resp_body.streams {
            if stream_id.contains(&streams.stream_id) {
                if streams.stream_status == "FAIL" || streams.stream_status == "ABORTED" {
                    is_fail = true;
                } else if streams.stream_status == "SUCCESS" {
                    is_success = true;
                } else {
                    is_fail=true;
                    batch_run_completed=false;
                    println!(
                        "StreamId {} is in {} status",
                        &streams.stream_id, &streams.stream_status
                    );
                }
            }
        }
        if batch_run_completed || retry_count >= max_retry {
            break;
        }
        else{
            retry_count = retry_count + 1;
            println!{" checking batch - {} again in : {} sec for {} time",batch_id,wait_time, retry_count};
            sleep(Duration::from_secs(wait_time));
        }

    }
    if is_success && !is_fail {
        true
    } else {
        false
    }
}

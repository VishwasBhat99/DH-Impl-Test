use serde::Serialize;

#[derive(Serialize, Debug)]
struct Body {
    batch_id: i64,
    as_on_date: String,
    status: String,
}

pub fn run(as_on_date: String, batch_id: i64, status: String, connection_string: &String) {
    let trigger_body = Body {
        batch_id: batch_id,
        as_on_date: as_on_date,
        status: status,
    };
    let url = format!("http://{}/update_batch_status_holiday/1", connection_string);
    let client = reqwest::Client::new();
    let resp = client.post(&url).json(&trigger_body).send();
    match &resp {
        Ok(val) => {
            println!("Success status: {:?}", val);
        }
        Err(err) => {
            println!("Could not send request: {}", err);
        }
    }
}

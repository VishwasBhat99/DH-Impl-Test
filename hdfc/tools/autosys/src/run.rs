use serde::Serialize;

#[derive(Serialize, Debug)]
struct TriggerBody {
    as_on_date: String,
    batch_id: i64,
    stream_ids: Vec<i64>
}

pub fn run(as_on_date: &String, batch_id: i64, stream_id: &Vec<i64>, connection_string: &String,accept_invalid_certs:&bool) {
    let trigger_body = TriggerBody {
        as_on_date: as_on_date.to_string(),
        batch_id: batch_id,
        stream_ids: stream_id.to_owned()
    };
    let url = format!("{}/trigger/1", connection_string);
    let client_builder = reqwest::ClientBuilder::new().danger_accept_invalid_certs(*accept_invalid_certs);
    let client = client_builder.build();
    let resp = client.unwrap().post(&url).json(&trigger_body).send();
    match &resp {
        Ok(val) => {
            println!("Trigger status: {:?}", val);
        }
        Err(err) => {
            println!("Could not trigger batch run: {}", err);
        }
    }
}

extern crate serde;
use run::run;

mod run;

fn main() {
    // Todo: Use CLAP argument parser
    let args: Vec<String> = std::env::args().collect();
    // Read all arguments and parse them
    let as_on_date = args[1].to_string();
    let batch_id = args[2]
        .parse::<i64>()
        .expect("Cannot parse batch id arg as integer");
    let status = args[3].to_string();
    let connection_string = &args[4];

    run(as_on_date, batch_id, status, connection_string);
}

use flow::get_flow_info;
use screens::display_flow_screen;
use std::io;
use structs::FlowDef;
use structs::StreamDef;

pub fn get_stream_info() -> StreamDef {
    let mut stream_name = String::new();
    println!("Enter Stream Name: ");
    io::stdin()
        .read_line(&mut stream_name)
        .expect("Error: Unable to read stream name from user.");
    let mut stream_id = String::new();
    println!("Enter Stream ID: ");
    io::stdin()
        .read_line(&mut stream_id)
        .expect("Error: Unable to read stream id from user.");
    let mut flows: Vec<FlowDef> = Vec::new();
    let mut choice = "1".to_string();
    while &choice != "3" {
        match choice.trim() {
            "1" => {
                println!("Current Flows in Stream: \n{:#?}", flows);
                choice = display_flow_screen();
            }
            "2" => {
                let flow = get_flow_info();
                flows.push(flow);
                choice = display_flow_screen();
            }
            "3" => {
                break;
            }
            _ => {
                println!("Invalid Choice!! Please select again.");
                choice = display_flow_screen();
            }
        }
    }
    StreamDef {
        streamName: stream_name.trim().to_string(),
        streamId: stream_id.trim().to_string(),
        flows: flows,
    }
}

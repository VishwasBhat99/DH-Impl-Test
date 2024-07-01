use process::get_process_info;
use screens::display_process_screen;
use std::io;
use structs::FlowDef;
use structs::ProcDef;

pub fn get_flow_info() -> FlowDef {
    let mut flow_name = String::new();
    println!("Enter Flow Name: ");
    io::stdin()
        .read_line(&mut flow_name)
        .expect("Error: Unable to read flow name from user.");
    let mut flow_id = String::new();
    println!("Enter Flow ID: ");
    io::stdin()
        .read_line(&mut flow_id)
        .expect("Error: Unable to read flow id from user.");
    let mut flow_dep_list = String::new();
    println!("Enter Flow Dependencies(comma separated): ");
    io::stdin()
        .read_line(&mut flow_dep_list)
        .expect("Error: Unable to read flow dependencies from user.");
    let mut flow_dep: Vec<String> = Vec::new();
    if flow_dep_list.trim() != "" {
        flow_dep = flow_dep_list
            .split(',')
            .map(|str| str.trim().to_string())
            .collect();
    }
    let mut executor_id = String::new();
    println!("Enter Executor ID: ");
    io::stdin()
        .read_line(&mut executor_id)
        .expect("Error: Unable to read executor id from user.");
    let mut processes: Vec<ProcDef> = Vec::new();
    let mut choice = "1".to_string();
    while &choice != "3" {
        match choice.trim() {
            "1" => {
                println!("Current Processes in Flow: \n{:#?}", processes);
                choice = display_process_screen();
            }
            "2" => {
                let process = get_process_info();
                processes.push(process);
                choice = display_process_screen();
            }
            "3" => {
                break;
            }
            _ => {
                println!("Invalid Choice!! Please select again.");
                choice = display_process_screen();
            }
        }
    }

    FlowDef {
        name: flow_name.trim().to_string(),
        flowId: flow_id.trim().to_string(),
        flowDependencies: flow_dep,
        executorID: executor_id.trim().to_string(),
        process: processes,
    }
}
